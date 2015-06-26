#[macro_use]
extern crate diecast;
extern crate diecast_live as live;
extern crate diecast_websocket as websocket;
extern crate diecast_git as git;
extern crate diecast_rss as rss;
extern crate diecast_handlebars as handlebars;
extern crate diecast_scss as scss;
extern crate diecast_versions as versions;

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

use std::path::{Path, PathBuf};

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
use diecast::util::handle::{bind, item};

mod markdown;
mod view;

fn main() {
    env_logger::init().unwrap();

    // TODO: run/store this in websocket handler?
    let ws_tx = websocket::init();

    let templates =
        Rule::named("templates")
        .pattern(glob!("templates/*.html"))
        .handler(chain![bind::each(item::read), handlebars::register_templates])
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
        .pattern(Glob::new("scss/**/*.scss").unwrap())
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
        .depends_on(&templates)
        .pattern(glob!("posts/*.markdown"))
        .handler(chain![
            bind::each(chain![item::read, item::parse_metadata]),
            bind::retain(item::publishable),
            bind::each(chain![
                item::date,
                markdown::markdown(),
                versions::save("rendered"),
                route::pretty]),
            bind::tags,
            websocket::pipe(ws_tx.clone()),
            git::git,
            bind::each(chain![
                handlebars::render(&templates, "post", view::post_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write]),
            bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
                b.cmp(a)
        })])
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
        .pattern(Glob::new("pages/*.markdown").unwrap())
        .depends_on(&templates)
        .handler(chain![
            bind::each(chain![
                item::read,
                item::parse_metadata]),
            bind::retain(item::publishable),
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
        .pattern(Glob::new("notes/*.markdown").unwrap())
        .depends_on(&templates)
        .handler(chain![
            bind::each(chain![
                item::read,
                item::parse_metadata]),
            bind::retain(item::publishable),
            bind::each(chain![
                item::date,
                // TODO: use pulldown instead for more cross-platform?
                markdown::markdown(),
                route::pretty]),
            websocket::pipe(ws_tx.clone()),
            git::git,
            bind::each(chain![
                handlebars::render(&templates, "note", view::post_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write]),
            bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
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

    // TODO: this should be expressed in such a way that it is possible to paginate
    let tags =
        Rule::named("tag index")
        .depends_on(&templates)
        .depends_on(&posts)
        .handler(chain![
            tag_index,
            bind::each(chain![
                handlebars::render(&templates, "tags", view::tags_index_template),
                handlebars::render(&templates, "layout", view::layout_template),
                item::write])])
        .build();

    let feed =
        Rule::named("feed")
        .depends_on(&posts)
        .handler(chain![
            rss::feed("rss.xml", "Blaenk Denum", "http://www.blaenkdenum.com", rss_handler),
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
            // TODO check Result

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

fn rss_handler(_title: &str, url: &str, bind: &Bind) -> Vec<rss::Item> {
    bind.dependencies["posts"].iter()
        .take(10)
        .map(|i| {
            let mut feed_item: rss::Item = Default::default();

            feed_item.pub_date =
                i.extensions.get::<item::Date>()
                .map(ToString::to_string);

            feed_item.description = versions::get(i, "rendered");

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

