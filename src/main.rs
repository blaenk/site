#[macro_use]
extern crate diecast;
extern crate diecast_websocket;
extern crate diecast_git;
extern crate diecast_rss;
extern crate diecast_handlebars;
extern crate diecast_scss;
// extern crate diecast_live;

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
use std::sync::{Arc, Mutex};
use rustc_serialize::json::Json;
use std::process::{Command, Child};

use time::PreciseTime;
use glob::Pattern as Glob;

use diecast::{
    CommandBuilder,
    Rule,
    Bind,
    Item,
};

use diecast::support;
use diecast::util::route;
use diecast::util::source;
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

    let mut pig_handle = pig();

    // TODO: run/store this in websocket handler?
    let ws_tx = diecast_websocket::init();

    println!("pig server initialized");

    let context = Arc::new(Mutex::new(zmq::Context::new()));

    let templates =
        Rule::matching("templates", "templates/*.html".parse::<Glob>().unwrap())
        .handler(Chain::new()
            .link(bind::parallel_each(item::read))
            .link(diecast_handlebars::register_templates))
        .build();

    let statics =
        Rule::matching("statics", or!(
            "images/**/*".parse::<Glob>().unwrap(),
            "static/**/*".parse::<Glob>().unwrap(),
            "js/**/*".parse::<Glob>().unwrap(),
            "favicon.png",
            "CNAME"
        ))
        .handler(bind::parallel_each(Chain::new()
            .link(route::identity)
            .link(item::copy)))
        .build();

    let scss =
        Rule::matching("scss", "scss/**/*.scss".parse::<Glob>().unwrap())
        .handler(diecast_scss::scss("scss/screen.scss", "css/screen.css"))
        .build();

    let posts =
        Rule::matching("posts", "posts/*.markdown".parse::<Glob>().unwrap())
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::parallel_each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)
                .link(item::date)))
            .link(bind::retain(item::publishable))
            // TODO need this:
            // .link(bind::cond(item::publishable, handler))
            .link(bind::parallel_each(Chain::new()
                .link(markdown::markdown(context.clone()))
                .link(item::save_version("rendered"))
                .link(route::pretty)))
            .link(bind::tags)
            .link(diecast_websocket::pipe(ws_tx.clone()))
            .link(diecast_git::git)
            .link(bind::parallel_each(Chain::new()
                .link(diecast_handlebars::render_template(&templates, "post", view::post_template))
                .link(diecast_handlebars::render_template(&templates, "layout", view::layout_template))
                .link(item::write)))
            .link(bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
                b.cmp(a)
            })))
        .build();

    let posts_index =
        Rule::creating("post index")
        .depends_on(&posts)
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::create("index.html"))
            .link(bind::each(Chain::new()
            .link(diecast_handlebars::render_template(&templates, "index", view::posts_index_template))
            .link(diecast_handlebars::render_template(&templates, "layout", view::layout_template))
            .link(item::write))))
        .build();

    let pages =
        Rule::matching("pages", "pages/*.markdown".parse::<Glob>().unwrap())
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::parallel_each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)))
            // TODO: replace with some sort of filter/only_if
            // .link(bind::retain(item::publishable))
            .link(bind::parallel_each(Chain::new()
                .link(markdown::markdown(context.clone()))
                .link(|item: &mut Item| -> diecast::Result {
                    item.route_with(|path: &Path| -> PathBuf {
                        let without = path.with_extension("");
                        let mut result = PathBuf::from(without.file_name().unwrap());
                        result.push("index.html");
                        result
                    });

                    Ok(())
                })))
            .link(diecast_websocket::pipe(ws_tx.clone()))
            .link(bind::parallel_each(Chain::new()
                .link(diecast_handlebars::render_template(&templates, "page", view::post_template))
                .link(diecast_handlebars::render_template(&templates, "layout", view::layout_template))
                .link(item::write))))
        .build();

    let notes =
        Rule::matching("notes", "notes/*.markdown".parse::<Glob>().unwrap())
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::parallel_each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)
                .link(item::date)))
            // TODO: replace with some sort of filter/only_if
            // .link(bind::retain(item::publishable))
            .link(bind::parallel_each(Chain::new()
                .link(markdown::markdown(context.clone()))
                .link(route::pretty)))
            .link(diecast_websocket::pipe(ws_tx.clone()))
            .link(diecast_git::git)
            .link(bind::parallel_each(Chain::new()
                .link(diecast_handlebars::render_template(&templates, "note", view::post_template))
                .link(diecast_handlebars::render_template(&templates, "layout", view::layout_template))
                .link(item::write)))
            .link(bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
                b.cmp(a)
            })))
        .build();

    let notes_index =
        Rule::creating("note index")
        .depends_on(&notes)
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::create("notes/index.html"))
            .link(bind::parallel_each(Chain::new()
            .link(diecast_handlebars::render_template(&templates, "index", view::notes_index_template))
            .link(diecast_handlebars::render_template(&templates, "layout", view::layout_template))
            .link(item::write))))
        .build();

    // TODO: this should be expressed in such a way that it is possible to paginate
    let tags =
        Rule::creating("tag index")
        .depends_on(&templates)
        .depends_on(&posts)
        .handler(Chain::new()
            .link(tag_index)
            .link(bind::parallel_each(Chain::new()
            .link(diecast_handlebars::render_template(&templates, "tags", view::tags_index_template))
            .link(diecast_handlebars::render_template(&templates, "layout", view::layout_template))
            .link(item::write))))
        .build();

    let feed =
        Rule::creating("feed")
        .depends_on(&posts)
        .handler(Chain::new()
            .link(bind::create("rss.xml"))
            .link(bind::each(Chain::new()
                .link(diecast_rss::rss)
                .link(item::write))))
        .build();

    let not_found =
        Rule::creating("404")
        .depends_on(&templates)
        .handler(Chain::new()
            .link(bind::create("404.html"))
            .link(bind::each(Chain::new()
                .link(diecast_handlebars::render_template(&templates, "404", |_| Json::Null))
                .link(diecast_handlebars::render_template(&templates, "layout", view::layout_template))
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

    // let config = Configuration::new();
    // let site = Site::new(rules, configuration);

    // TODO separate it into two steps?
    // let mut command = command::from_args(rules, config);

    let command =
        CommandBuilder::new()
        // .plugin(diecast_live::plugin())
        .rules(rules)
        .build();

    match command {
        Ok(mut cmd) => {
            // TODO this time keeping should be moved to build
            let start = PreciseTime::now();
            cmd.run();
            let end = PreciseTime::now();
            println!("time elapsed: {}", start.to(end));
        },
        Err(e) => println!("command creation failed: {}", e),
    }

    pig_handle.kill().unwrap();
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

