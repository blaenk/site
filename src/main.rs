#[macro_use]
extern crate diecast;
extern crate diecast_date as date;
extern crate diecast_feed as feed;
extern crate diecast_git as git;
extern crate diecast_github_pages as github_pages;
extern crate diecast_handlebars as handlebars;
extern crate diecast_live as live;
extern crate diecast_metadata as metadata;
extern crate diecast_scss as scss;
extern crate diecast_tags as tags;
extern crate diecast_versions as versions;
extern crate diecast_websocket as websocket;

#[macro_use]
extern crate hoedown;

#[macro_use]
extern crate log;

extern crate chrono;
extern crate env_logger;
extern crate glob;
extern crate regex;
extern crate rss;
extern crate rustc_serialize;
extern crate sha1;
extern crate time;
extern crate toml;
extern crate typemap;
extern crate zmq_rs as zmq;

extern crate futures_cpupool;

use time::PreciseTime;

use futures_cpupool::CpuPool;

use diecast::{Site, Rule, Item};
use diecast::command;
use diecast::util::route;
use diecast::util::handle::{handle_if, bind, item};

use github_pages::{GitHubPages, BuildFrom};
use git::GitCommit;

// item: read, write, copy
// bind: each, retain, sort_by

mod markdown;
mod view;
mod helpers;

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

    let md = markdown::markdown();
    let ws_tx = websocket::init();

    let pool = bind::PooledEach::new(CpuPool::new(2));

    let templates =
        Rule::named("templates")
        .handler(chain![
            bind::select(glob!("templates/*.html")),
            pool.each(item::read),
            handlebars::register_templates
        ])
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
                "CNAME")
            ),
            pool.each(chain![
                route::identity,
                item::copy
            ])
        ])
        .build();

    let scss =
        Rule::named("scss")
        .handler(chain![
            bind::select(glob!("scss/**/*.scss")),
            scss::scss("scss/screen.scss", "css/screen.css")
        ])
        .build();

    let posts =
        Rule::named("posts")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("posts/*.markdown")),
            pool.each(chain![item::read, metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                md.clone(),
                handle_if(helpers::is_pushable, websocket::pipe(ws_tx.clone())),
                // this is used by feed
                versions::save("rendered"),
                route::pretty
            ]),
            tags::collect(|item: &Item| -> Vec<String> {
                item.extensions.get::<metadata::toml::Metadata>()
                .and_then(|m| m.lookup("tags"))
                .and_then(toml::Value::as_slice)
                .map_or(Vec::new(),
                    |s| s.iter()
                        .filter_map(toml::Value::as_str)
                        .map(String::from)
                        .collect()
                )
            }),
            GitCommit::from_revision("origin/master"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::post_template),
                item::write
            ]),
            bind::sort_by(helpers::publish_date_sort)
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
                item::write
            ])
        ])
        .build();

    let pages =
        Rule::named("pages")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("pages/*.markdown")),
            pool.each(chain![
                item::read,
                metadata::toml::parse
            ]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                md.clone(),
                handle_if(helpers::is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty_page
            ]),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::page_template),
                item::write
            ])
        ])
        .build();

    let notes =
        Rule::named("notes")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("notes/*.markdown")),
            pool.each(chain![
                item::read,
                metadata::toml::parse
            ]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                md.clone(),
                handle_if(helpers::is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty
            ]),
            GitCommit::from_revision("origin/master"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::note_template),
                item::write
            ]),
            bind::sort_by(helpers::title_sort)
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
                metadata::toml::parse
            ]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                md.clone(),
                handle_if(helpers::is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty
            ]),
            GitCommit::from_revision("origin/master"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::work_template),
                item::write
            ]),
            bind::sort_by(helpers::title_sort)])
        .build();

    let notes_index =
        Rule::named("note index")
        .depends_on(&notes)
        .depends_on(&templates)
        .handler(chain![
            bind::create("notes/index.html"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::notes_index_template),
                item::write
            ])
        ])
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
                item::write
            ])
        ])
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
                item::write
            ])
        ])
        .build();

    let feed =
        Rule::named("feed")
        .depends_on(&posts)
        .handler(chain![
            feed::rss::create("rss.xml",
                "Blaenk Denum",
                "http://www.blaenkdenum.com",
                helpers::rss_handler),
            pool.each(item::write)
        ])
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
                item::write
            ])
        ])
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
        .command("deploy",
                 GitHubPages::new("git@github.com:blaenk/blaenk.github.io.git", "master")
                 .build_from(BuildFrom::Revision(String::from("origin/master"))))
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
