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
extern crate diecast_github_pages as github_pages;

#[macro_use]
extern crate lazy_static;

extern crate libc;

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
extern crate zmq_rs as zmq;
extern crate sha1;
extern crate syncbox;

use std::cmp;

use time::PreciseTime;
use syncbox::ThreadPool;

use github_pages::GitHubPages;

use diecast::{Site, Rule, Item};
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

fn is_pushable(item: &Item) -> bool {
    item.extensions.get::<metadata::toml::Metadata>()
    .and_then(|m| m.lookup("push").and_then(toml::Value::as_bool))
    .unwrap_or(true)
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

    // TODO
    // these things only need to be run if the site is actually going to be run
    // perhaps a conditional
    //
    // site.before_build(|| {
    //   pig_handle = pig();
    //   context = Arc::new(Mutex::new(zmq::Context::new().unwrap()));
    //   ws_tx = websocket::init();
    //   pool = bind::PooledEach::new(ThreadPool::fixed_size(2));
    // })

    let md = markdown::markdown();

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
        .handler(chain![
            bind::select(glob!("templates/*.html")),
            pool.each(item::read),
            handlebars::register_templates])
        .build();

    let statics =
        Rule::named("statics")
        .handler(chain![
            bind::select(or!(
                glob!("images/**/*"),
                glob!("images/**/*"),
                glob!("static/**/*"),
                glob!("js/**/*"),
                "favicon.png",
                "CNAME")),
            pool.each(chain![
                route::identity,
                item::copy])])
        .build();

    let scss =
        Rule::named("scss")
        .handler(chain![
            bind::select(glob!("scss/**/*.scss")),
            scss::scss("scss/screen.scss", "css/screen.css")])
        .build();

    fn publish_date_sort(a: &Item, b: &Item) -> cmp::Ordering {
        let a = a.extensions.get::<PublishDate>().unwrap();
        let b = b.extensions.get::<PublishDate>().unwrap();
        b.cmp(a)
    }

    fn title_sort(a: &Item, b: &Item) -> cmp::Ordering {
        let a = a.extensions.get::<metadata::toml::Metadata>().unwrap();
        let b = b.extensions.get::<metadata::toml::Metadata>().unwrap();
        let a_title = a.lookup("title").unwrap().as_str().unwrap();
        let b_title = b.lookup("title").unwrap().as_str().unwrap();
        a_title.cmp(b_title)
    }

    let posts =
        Rule::named("posts")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("posts/*.markdown")),
            pool.each(chain![item::read, metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                // markdown::markdown(context.clone()),
                md.clone(),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                // this is used by feed
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
                handlebars::render(&templates, "layout", view::post_template),
                item::write]),
            bind::sort_by(publish_date_sort)
        ])
        .build();

    let posts_index =
        Rule::named("post index")
        .depends_on(&posts)
        .depends_on(&templates)
        .handler(chain![
            bind::create("index.html"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::posts_index_template),
                item::write])])
        .build();

    let pages =
        Rule::named("pages")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("pages/*.markdown")),
            pool.each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                // markdown::markdown(context.clone()),
                md.clone(),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty_page]),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::page_template),
                item::write])])
        .build();

    let notes =
        Rule::named("notes")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("notes/*.markdown")),
            pool.each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                // markdown::markdown(context.clone()),
                md.clone(),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty]),
            git::git,
            pool.each(chain![
                handlebars::render(&templates, "layout", view::note_template),
                item::write]),
            bind::sort_by(title_sort)
        ])
        .build();

    // TODO
    // find a way to DRY this, it's basically
    // identical to the `notes` rule
    let work =
        Rule::named("work")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("work/*.markdown")),
            pool.each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                // markdown::markdown(context.clone()),
                md.clone(),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty]),
            git::git,
            pool.each(chain![
                handlebars::render(&templates, "layout", view::work_template),
                item::write]),
            bind::sort_by(title_sort)])
        .build();

    let notes_index =
        Rule::named("note index")
        .depends_on(&notes)
        .depends_on(&templates)
        .handler(chain![
            bind::create("notes/index.html"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::notes_index_template),
                item::write])])
        .build();

    let work_index =
        Rule::named("work index")
        .depends_on(&work)
        .depends_on(&templates)
        .handler(chain![
            bind::create("work/index.html"),
            pool.each(chain![
                handlebars::render(&templates, "layout",
                                   view::work_index_template),
                item::write])])
        .build();

    // TODO
    // should be able to sort tags here
    let tags =
        Rule::named("tag index")
        .depends_on(&templates)
        .depends_on(&posts)
        .handler(chain![
            helpers::tag_index,
            pool.each(chain![
                handlebars::render(&templates, "layout",
                                   view::tags_index_template),
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
        .handler(chain![
            bind::select("404.html"),
            pool.each(chain![
                item::read,
                route::identity,
                handlebars::render(&templates, "layout", view::with_body),
                item::write])])
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
        work,
        work_index,
        feed,
        not_found,
    ];

    // TODO
    // I think I'd prefer site.add_rule(Rule::etc)
    // the problem (?) is that can no longer .depends_on(&rule)
    let mut site = Site::new(rules);

    let command =
        command::Builder::new()
        .command("live", live::Live::new("0.0.0.0:4000"))
        // TODO
        // if config.output dir doesn't exist, site.build()?
        .command("deploy",
                 GitHubPages::new("git@github.com:blaenk/blaenk.github.io.git",
                                  "master")
                 .git(".testing.git"))
        .build();

    match command {
        Ok(mut cmd) => {
            // TODO
            // this time keeping should be moved to build
            // especially because it doesn't make much sense
            // to time certain things, like 'live'?
            let start = PreciseTime::now();

            match cmd.run(&mut site) {
                Ok(()) => (),
                Err(e) => println!("command execution failed: {}", e),
            }

            let end = PreciseTime::now();
            println!("time elapsed: {}", start.to(end));
        },
        Err(e) => println!("command creation failed: {}", e),
    }

    // pig_handle.kill().unwrap();
}
