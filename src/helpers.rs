use std::path::Path;

use diecast::{self, Bind, Item};
use diecast::support;

use rss;
use toml;
use chrono;

use metadata;
use tags;
use versions;

use super::PublishDate;

fn get_published(item: &Item) -> Option<&str> {
    item.extensions.get::<metadata::toml::Metadata>()
    .and_then(|meta| meta.lookup("published").and_then(toml::Value::as_str))
}

fn date_handler(item: &Item) -> diecast::Result<chrono::NaiveDate> {
    if let Some(meta) = item.extensions.get::<metadata::toml::Metadata>() {
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

pub fn set_date(item: &mut Item) -> diecast::Result<()> {
    let date = try!(date_handler(item));

    item.extensions.insert::<PublishDate>(date);

    Ok(())
}

// TODO can't depend on Metadata or "draft"
// NOTE
// with this you can put drafts wherever, e.g. posts/drafts/blah.md
// provided they have draft = true if they are drafts
pub fn is_draft(item: &Item) -> bool {
    item.extensions.get::<metadata::toml::Metadata>()
    .map_or(false, |meta| {
        meta.lookup("draft")
        .and_then(::toml::Value::as_bool)
        .unwrap_or(false)
    })
}

pub fn publishable(item: &Item) -> bool {
    !(is_draft(item) && !item.bind().configuration.is_preview)
}

pub fn tag_index(bind: &mut Bind) -> diecast::Result<()> {
    let mut items = vec![];

    if let Some(tags) = bind.dependencies["posts"].extensions.read().unwrap().get::<tags::Tags>() {
        for (tag, itms) in tags {
            let url = support::slugify(&tag);

            let mut item = Item::writing(format!("tags/{}/index.html", url));

            item.extensions.insert::<tags::Tag>(tags::Tag {
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

pub fn rss_handler(_title: &str, url: &str, bind: &Bind) -> Vec<rss::Item> {
    bind.dependencies["posts"].iter()
        .take(10)
        .map(|i| {
            let mut feed_item: rss::Item = Default::default();

            feed_item.pub_date =
                i.extensions.get::<PublishDate>()
                .map(ToString::to_string);

            feed_item.description = versions::get(i, "rendered");

            if let Some(meta) = i.extensions.get::<metadata::toml::Metadata>() {
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
