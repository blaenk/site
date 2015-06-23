#[macro_use]
extern crate diecast;
extern crate diecast_live as live;
extern crate diecast_websocket as websocket;
extern crate diecast_git as git;
extern crate diecast_rss as rss;
extern crate diecast_handlebars as handlebars;
extern crate diecast_scss as scss;

#[macro_use]
extern crate hoedown;

#[macro_use]
extern crate log;

extern crate env_logger;
extern crate glob;
extern crate zmq;
extern crate regex;
extern crate toml;
extern crate rustc_serialize;
extern crate time;
extern crate typemap;

use std::path::{Path, PathBuf};
use std::process::{Command, Child};
use std::sync::{Arc, Mutex};

use glob::Pattern as Glob;
use rustc_serialize::json::Json;
use time::PreciseTime;

use diecast::{
    Rule,
    Bind,
    Item,
};

use diecast::{command, support};
use diecast::util::{route, source};
use diecast::util::handle::{Chain, bind, item};

mod markdown;
mod view;

// TODO: implement some sort of heartbeat so that the pig
// server dies when this process dies
fn pig() -> Child {
    println!("initializing pig server...");

    Command::new("python")
        .arg("scripts/pig.py")
        .spawn()
        .unwrap()
}

fn main() {
    env_logger::init().unwrap();

    // TODO: run/store this in websocket handler?
    let ws_tx = websocket::init();

    let templates =
        Rule::named("templates")
        .matching(Glob::new("templates/*.html").unwrap())
        .handler(Chain::new()
            .link(bind::each(item::read))
            .link(handlebars::register_templates))
        .build();

    let statics =
        Rule::named("statics")
        .matching(or!(
            Glob::new("images/**/*").unwrap(),
            Glob::new("static/**/*").unwrap(),
            Glob::new("js/**/*").unwrap(),
            "favicon.png",
            "CNAME"
        ))
        .handler(bind::each(Chain::new()
            .link(route::identity)
            .link(item::copy)))
        .build();

    let scss =
        Rule::named("scss")
        .matching(Glob::new("scss/**/*.scss").unwrap())
        // TODO: use Item::spawn here too
        .handler(scss::scss("scss/screen.scss", "css/screen.css"))
        .build();

    // TODO
    // one possible way to avoid Matching/Creating
    // would be to keep track of all Reading(path)s for each Bind
    // and when a file in those sets changes trigger resolve_from(the_bind)
    // if a file is not in the sets, then re-build()?
    //
    // update if:
    // * known file changes
    //
    // re-build if:
    // * known file is removed
    // * unknown file's event
    //
    // cost:
    // * everything is unnecessarily rebuilt if it turns out that no one
    //   is interested in the event path
    // * full rebuild when unknown-but-matching path is changed when a simple
    //   update() would have sufficed

    let posts =
        Rule::named("posts")
        .matching(Glob::new("posts/*.markdown").unwrap())
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)))
            .link(bind::retain(item::publishable))
            .link(bind::each(Chain::new()
                .link(item::date)
                .link(markdown::markdown())
                .link(item::save_version("rendered"))
                .link(route::pretty)))
            .link(bind::tags)
            .link(websocket::pipe(ws_tx.clone()))
            .link(git::git)
            .link(bind::each(Chain::new()
                .link(handlebars::render(&templates, "post", view::post_template))
                .link(handlebars::render(&templates, "layout", view::layout_template))
                .link(item::write)))
            .link(bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
                b.cmp(a)
            })))
        .build();

    let posts_index =
        Rule::named("post index")
        .depends_on(&posts)
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::create("index.html"))
            .link(bind::each(Chain::new()
                .link(handlebars::render(&templates, "index", view::posts_index_template))
                .link(handlebars::render(&templates, "layout", view::layout_template))
                .link(item::write))))
        .build();

    let pages =
        Rule::named("pages")
        .matching(Glob::new("pages/*.markdown").unwrap())
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)))
            .link(bind::retain(item::publishable))
            .link(bind::each(Chain::new()
                .link(markdown::markdown())
                .link(route::pretty_page)))
            .link(websocket::pipe(ws_tx.clone()))
            .link(bind::each(Chain::new()
                .link(handlebars::render(&templates, "page", view::post_template))
                .link(handlebars::render(&templates, "layout", view::layout_template))
                .link(item::write))))
        .build();

    let notes =
        Rule::named("notes")
        .matching(Glob::new("notes/*.markdown").unwrap())
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)))
            .link(bind::retain(item::publishable))
            .link(bind::each(Chain::new()
                .link(item::date)
                // TODO: use pulldown instead for more cross-platform?
                .link(markdown::markdown())
                .link(route::pretty)))
            .link(websocket::pipe(ws_tx.clone()))
            .link(git::git)
            .link(bind::each(Chain::new()
                .link(handlebars::render(&templates, "note", view::post_template))
                .link(handlebars::render(&templates, "layout", view::layout_template))
                .link(item::write)))
            .link(bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
                b.cmp(a)
            })))
        .build();

    let notes_index =
        Rule::named("note index")
        .depends_on(&notes)
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::create("notes/index.html"))
            .link(bind::each(Chain::new()
                .link(handlebars::render(&templates, "index", view::notes_index_template))
                .link(handlebars::render(&templates, "layout", view::layout_template))
                .link(item::write))))
        .build();

    // TODO: this should be expressed in such a way that it is possible to paginate
    let tags =
        Rule::named("tag index")
        .depends_on(&templates)
        .depends_on(&posts)
        .handler(Chain::new()
            .link(tag_index)
            .link(bind::each(Chain::new()
                .link(handlebars::render(&templates, "tags", view::tags_index_template))
                .link(handlebars::render(&templates, "layout", view::layout_template))
                .link(item::write))))
        .build();

    let feed =
        Rule::named("feed")
        .depends_on(&posts)
        .handler(Chain::new()
            .link(rss::feed("rss.xml", "Blaenk Denum", "http://www.blaenkdenum.com", feed_handler))
            .link(bind::each(item::write)))
        .build();

    // TODO
    // change this to a ReadWrite
    // read 404.markdown
    // render(layout)
    // output 404.html
    let not_found =
        Rule::named("404")
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::create("404.html"))
            .link(bind::each(Chain::new()
                // TODO: just read 404.html
                .link(handlebars::render(&templates, "404", |_| Json::Null))
                .link(handlebars::render(&templates, "layout", view::layout_template))
                .link(item::write))))
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
            // TODO check Result
            cmd.run();
            let end = PreciseTime::now();
            println!("time elapsed: {}", start.to(end));
        },
        Err(e) => println!("command creation failed: {}", e),
    }

    // pig_handle.kill().unwrap();
}

fn tag_index(bind: &mut Bind) -> diecast::Result {
    let mut items = vec![];

    if let Some(tags) = bind.dependencies["posts"].extensions.read().unwrap().get::<bind::Tags>() {
        for (tag, itms) in tags {
            let url = support::slugify(&tag);

            let mut pgs = source::pages(&bind.dependencies["posts"], 5, &move |page: usize| -> PathBuf {
                if page == 0 {
                    PathBuf::from(&format!("tags/{}/index.html", url))
                } else {
                    PathBuf::from(&format!("tags/{}/{}/index.html", url, page))
                }
            });

            for item in &mut pgs {
                item.extensions.insert::<view::Tag>(view::Tag { tag: tag.clone(), items: itms.clone() });
            }

            items.extend(pgs);
        }
    }

    bind.items_mut().extend(items.into_iter());

    Ok(())
}

fn feed_handler(title: &str, url: &str, bind: &Bind) -> Vec<rss::Item> {
    bind.dependencies["posts"].iter()
        .take(10)
        .map(|i| {
            let mut feed_item: rss::Item = Default::default();

            feed_item.pub_date =
                i.extensions.get::<item::Date>()
                .map(ToString::to_string);

            feed_item.description =
                i.extensions.get::<item::Versions>()
                .and_then(|versions| versions.get("rendered").map(Clone::clone));

            if let Some(meta) = i.extensions.get::<item::Metadata>() {
                feed_item.title =
                    meta.lookup("title")
                    .and_then(toml::Value::as_str)
                    .map(String::from);

                feed_item.link =
                    i.route().writing()
                    .and_then(Path::parent)
                    .and_then(Path::to_str)
                    .map(|p| format!("{}/{}", url, p));
            }

            feed_item
        })
    .collect()
}

