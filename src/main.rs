#[macro_use]
extern crate diecast;
extern crate diecast_live as live;
extern crate diecast_websocket as websocket;
extern crate diecast_git as git;
extern crate diecast_rss as dc_rss;
extern crate diecast_handlebars as handlebars;
extern crate diecast_scss as scss;
extern crate diecast_versions as versions;
extern crate diecast_toml as dc_toml;
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

use std::path::Path;
use std::error::Error as StdError;

use time::PreciseTime;

use diecast::{
    Rule,
    Bind,
    Item,
};

use diecast::{command, support};
use diecast::util::route;
use diecast::util::handle::{bind, item};

// TODO can't depend on Metadata or "draft"
pub fn is_draft(item: &Item) -> bool {
    item.extensions.get::<dc_toml::Metadata>()
    .map_or(false, |meta| {
        meta.lookup("draft")
        .and_then(::toml::Value::as_bool)
        .unwrap_or(false)
    })
}

pub fn publishable(item: &Item) -> bool {
    !(is_draft(item) && !item.bind().configuration.is_preview)
}

pub struct PublishDate;

impl typemap::Key for PublishDate {
    type Value = chrono::NaiveDate;
}

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
        .pattern(glob!("scss/**/*.scss"))
        // TODO: use Item::spawn here too
        .handler(scss::scss("scss/screen.scss", "css/screen.css"))
        .build();

    // TODO
    // to abstract this:
    //
    // * metadata fetch {extension key, meta key}
    // * date type

    // TODO
    //
    // what we want is some way to register 'acessors' for common types
    // of data.
    //
    // for example, we would register an accessor function for
    // e.g. retrieving the Metadata, since it would either be
    // toml::Metadata, json::Metadata, or yaml::Metadata, etc.
    //
    // this would not suffice because each Metadata has a different
    // way of handling things
    //
    // item.register("published", get_published);
    // item.access("published");

    fn get_published(item: &Item) -> Option<&str> {
        item.extensions.get::<dc_toml::Metadata>()
        .and_then(|meta| meta.lookup("published").and_then(toml::Value::as_str))
    }

    fn date_handler(item: &Item) -> Result<chrono::NaiveDate, diecast::Error> {
        // item.extensions.get::<dc_toml::Metadata>()
        // .and_then(|meta|
        //     meta.lookup("published")
        //     .map_or(
        //         Err(From::from(
        //             format!("[date] No 'published' field in metadata for {:?}", item))),
        //         toml::Value::as_str))

        if let Some(meta) = item.extensions.get::<dc_toml::Metadata>() {
            if let Some(date) = meta.lookup("published").and_then(toml::Value::as_str) {
                chrono::NaiveDate::parse_from_str(date, "%B %e, %Y").map_err(From::from)
            } else {
                Err(From::from(
                    format!("[date] No 'published' field in metadata for {:?}", item)))
            }
        } else {
            Err(From::from(format!("[date] No metadata for {:?}", item)))
        }
    }

    fn date(item: &mut Item) -> diecast::Result {
        let date = try!(date_handler(item));

        item.extensions.insert::<PublishDate>(date);

        Ok(())
    }

    let posts =
        Rule::named("posts")
        .depends_on(&templates)
        .pattern(glob!("posts/*.markdown"))
        .handler(chain![
            bind::each(chain![item::read, dc_toml::parse]),
            bind::retain(publishable),
            bind::each(chain![
                date,
                markdown::markdown(),
                versions::save("rendered"),
                route::pretty]),
            tags::collect(|item: &Item| -> Vec<String> {
                item.extensions.get::<dc_toml::Metadata>()
                .and_then(|m| m.lookup("tags"))
                .and_then(toml::Value::as_slice)
                // TODO
                // filter_map would subtly ignore non-str tags
                // should it be unwrap instead?
                .map_or(Vec::new(),
                    |s| s.iter().filter_map(toml::Value::as_str).map(String::from).collect())
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
                dc_toml::parse]),
            bind::retain(publishable),
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
                dc_toml::parse]),
            bind::retain(publishable),
            bind::each(chain![
                date,
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
            dc_rss::feed("rss.xml", "Blaenk Denum", "http://www.blaenkdenum.com", rss_handler),
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

    if let Some(tags) = bind.dependencies["posts"].extensions.read().unwrap().get::<tags::Tags>() {
        for (tag, itms) in tags {
            let url = support::slugify(&tag);

            let mut item = Item::writing(format!("tags/{}/index.html", url));

            item.extensions.insert::<view::Tag>(view::Tag {
                tag: tag.clone(),
                items: itms.clone()
            });

            items.push(item);
        }
    }

    for item in items {
        bind.attach(item);
    }

    Ok(())
}

fn rss_handler(_title: &str, url: &str, bind: &Bind) -> Vec<rss::Item> {
    bind.dependencies["posts"].iter()
        .take(10)
        .map(|i| {
            let mut feed_item: rss::Item = Default::default();

            feed_item.pub_date =
                i.extensions.get::<PublishDate>()
                .map(ToString::to_string);

            feed_item.description = versions::get(i, "rendered");

            if let Some(meta) = i.extensions.get::<dc_toml::Metadata>() {
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

