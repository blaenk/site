#[macro_use]
extern crate diecast;
extern crate diecast_live as live;
extern crate diecast_websocket as websocket;
extern crate diecast_git as git;
extern crate diecast_rss as dc_rss;
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

use time::PreciseTime;

use diecast::{Rule, Bind, Item};
use diecast::command;
use diecast::util::route;
use diecast::util::handle::{bind, item};

mod markdown;
mod view;
mod helpers;

pub struct PublishDate;

impl typemap::Key for PublishDate {
    type Value = chrono::NaiveDate;
}

fn main() {
    env_logger::init().unwrap();

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

    let templates =
        Rule::named("templates")
        .pattern(glob!("templates/*.html"))
        .handler(chain![
            bind::each(item::read),
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
        .handler(bind::each(chain![route::identity, item::copy]))
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
            bind::each(chain![item::read, metadata::toml::parse]),
            bind::retain(helpers::publishable),
            bind::each(chain![
                helpers::set_date,
                markdown::markdown(),
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
            websocket::pipe(ws_tx.clone()),
            git::git,
            bind::each(chain![
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
            bind::each(chain![
                handlebars::render(&templates, "index", view::posts_index_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let pages =
        Rule::named("pages")
        .pattern(glob!("pages/*.markdown"))
        .depends_on(&templates)
        .handler(chain![
            bind::each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            bind::each(chain![
                markdown::markdown(),
                route::pretty_page]),
            websocket::pipe(ws_tx.clone()),
            bind::each(chain![
                handlebars::render(&templates, "page", view::post_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let notes =
        Rule::named("notes")
        .pattern(glob!("notes/*.markdown"))
        .depends_on(&templates)
        .handler(chain![
            bind::each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            bind::each(chain![
                helpers::set_date,
                markdown::markdown(),
                route::pretty]),
            websocket::pipe(ws_tx.clone()),
            git::git,
            bind::each(chain![
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
            bind::each(chain![
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
            bind::each(chain![
                handlebars::render(&templates, "tags", view::tags_index_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let feed =
        Rule::named("feed")
        .depends_on(&posts)
        .handler(chain![
            dc_rss::feed("rss.xml",
                "Blaenk Denum",
                "http://www.blaenkdenum.com",
                helpers::rss_handler),
            bind::each(item::write)])
        .build();

    let not_found =
        Rule::named("404")
        .depends_on(&templates)
        .pattern("404.html")
        .handler(bind::each(chain![
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
}

