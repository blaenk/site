#[macro_use]
extern crate diecast;
extern crate diecast_live as live;
extern crate diecast_websocket as websocket;
extern crate diecast_git as git;
extern crate diecast_feed as feed;
extern crate diecast_handlebars as handlebars;
extern crate diecast_scss as scss;
extern crate diecast_versions as versions;
extern crate diecast_metadata as metadata;
extern crate diecast_date as date;
extern crate diecast_tags as tags;

#[macro_use]
extern crate hoedown;

#[macro_use]
extern crate log;

extern crate env_logger;
extern crate glob;
extern crate regex;
extern crate toml;
extern crate rustc_serialize;
extern crate time;
extern crate typemap;
extern crate chrono;
extern crate rss;
extern crate zmq;
extern crate sha1;
extern crate syncbox;

use std::process::{Command, Child};
use std::sync::{Arc, Mutex};

use time::PreciseTime;
use syncbox::ThreadPool;

use diecast::{Rule, Bind, Item};
use diecast::command;
use diecast::util::route;
use diecast::util::handle::{handle_if, bind, item};

// item: read, write, copy
// bind: each, retain, sort_by

mod markdown;
mod view;
mod helpers;

pub struct PublishDate;

impl typemap::Key for PublishDate {
    type Value = chrono::NaiveDate;
}

fn pig() -> Child {
    println!("initializing pig server...");

    Command::new("python")
        .arg("scripts/pig.py")
        .spawn()
        .unwrap()
}

fn is_pushable(item: &Item) -> bool {
    item.extensions.get::<metadata::toml::Metadata>()
    .and_then(|m| m.lookup("push").and_then(toml::Value::as_bool))
    .unwrap_or(false)
}

// TODO
//
// thread budget:
//
// total overall: 20
//
// total site: 12
//
// * websocket::init [2]
// * evaluator [4]
// * each pool [4]
// * zmq (zmq::Context::socket; src/markdown.rs:193; zmq/src/lib.rs:270) [2]
//
// total separate: 8
// * subtract: pygments [6]
// * subtract: py-zmq [2

fn main() {
    env_logger::init().unwrap();

    let mut pig_handle = pig();

    // initialize_pool!(4).unwrap();

    let context = Arc::new(Mutex::new(zmq::Context::new()));

    // TODO
    // run/store this in websocket handler?
    // advantages: simpler setup
    // disadvantages: can't share the ws_tx
    //
    // work-around: store the ws_tx in arc-mutex in handler
    // disadvantage: lock contention when it can easily be cloned
    //
    // work-around: make websocket cloneable
    let ws_tx = websocket::init();

    let pool = bind::PooledEach::new(ThreadPool::fixed_size(2));

    let templates =
        Rule::named("templates")
        .pattern(glob!("templates/*.html"))
        .handler(chain![
            pool.each(item::read),
            handlebars::register_templates])
        .build();

    let statics =
        Rule::named("statics")
        .pattern(or!(
            glob!("images/**/*"),
            glob!("images/**/*"),
            glob!("static/**/*"),
            glob!("js/**/*"),
            "favicon.png",
            "CNAME"
        ))
        .handler(pool.each(chain![route::identity, item::copy]))
        .build();

    let scss =
        Rule::named("scss")
        .pattern(glob!("scss/**/*.scss"))
        .handler(scss::scss("scss/screen.scss", "css/screen.css"))
        .build();

    // let scss =
    //     Rule::named("scss")
    //     .pattern(glob!("scss/**/*.scss"))
    //     .build();

    // let css =
    //     Rule::named("css")
    //     .depends_on(&scss)
    //     .handler(chain![
    //         scss::compiler("scss/screen.scss", "css/screen.css"),
    //         item::write])
    //     .build();

    let posts =
        Rule::named("posts")
        .depends_on(&templates)
        .pattern(glob!("posts/*.markdown"))
        .handler(chain![
            pool.each(chain![item::read, metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                markdown::markdown(context.clone()),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                versions::save("rendered"),
                route::pretty]),
            tags::collect(|item: &Item| -> Vec<String> {
                item.extensions.get::<metadata::toml::Metadata>()
                .and_then(|m| m.lookup("tags"))
                .and_then(toml::Value::as_slice)
                .map_or(Vec::new(),
                    |s| s.iter()
                        .filter_map(toml::Value::as_str)
                        .map(String::from)
                        .collect())
            }),
            git::git,
            pool.each(chain![
                handlebars::render(&templates, "post", view::post_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write]),
            bind::sort_by(|a, b| {
                let a = a.extensions.get::<PublishDate>().unwrap();
                let b = b.extensions.get::<PublishDate>().unwrap();
                b.cmp(a)
            })
        ])
        .build();

    let posts_index =
        Rule::named("post index")
        .depends_on(&posts)
        .depends_on(&templates)
        .handler(chain![
            bind::create("index.html"),
            pool.each(chain![
                handlebars::render(&templates, "index", view::posts_index_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let pages =
        Rule::named("pages")
        .pattern(glob!("pages/*.markdown"))
        .depends_on(&templates)
        .handler(chain![
            pool.each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                markdown::markdown(context.clone()),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty_page]),
            pool.each(chain![
                handlebars::render(&templates, "page", view::post_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let notes =
        Rule::named("notes")
        .pattern(glob!("notes/*.markdown"))
        .depends_on(&templates)
        .handler(chain![
            pool.each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                markdown::markdown(context.clone()),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty]),
            git::git,
            pool.each(chain![
                handlebars::render(&templates, "note", view::post_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write]),
            bind::sort_by(|a, b| {
                let a = a.extensions.get::<PublishDate>().unwrap();
                let b = b.extensions.get::<PublishDate>().unwrap();
                b.cmp(a)
            })])
        .build();

    let notes_index =
        Rule::named("note index")
        .depends_on(&notes)
        .depends_on(&templates)
        .handler(chain![
            bind::create("notes/index.html"),
            pool.each(chain![
                handlebars::render(&templates, "index", view::notes_index_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let tags =
        Rule::named("tag index")
        .depends_on(&templates)
        .depends_on(&posts)
        .handler(chain![
            helpers::tag_index,
            pool.each(chain![
                handlebars::render(&templates, "tags", view::tags_index_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let feed =
        Rule::named("feed")
        .depends_on(&posts)
        .handler(chain![
            feed::rss::create("rss.xml",
                "Blaenk Denum",
                "http://www.blaenkdenum.com",
                helpers::rss_handler),
            pool.each(item::write)])
        .build();

    let not_found =
        Rule::named("404")
        .depends_on(&templates)
        .pattern("404.html")
        .handler(pool.each(chain![
            item::read,
            route::identity,
            handlebars::render(&templates, "layout", view::layout_template),
            item::write]))
        .build();

    let rules = vec![
        templates,
        statics,
        scss,
        pages,
        posts,
        posts_index,
        tags,
        notes,
        notes_index,
        feed,
        not_found,
    ];

    let command =
        command::Builder::new()
        .plugin(live::plugin())
        .rules(rules)
        .build();

    match command {
        Ok(mut cmd) => {
            // TODO this time keeping should be moved to build
            let start = PreciseTime::now();

            match cmd.run() {
                Ok(()) => (),
                Err(e) => println!("command execution failed: {}", e),
            }

            let end = PreciseTime::now();
            println!("time elapsed: {}", start.to(end));
        },
        Err(e) => println!("command creation failed: {}", e),
    }

    pig_handle.kill().unwrap();
}

