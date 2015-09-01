use std::path::Path;
use std::collections::BTreeMap;

use toml;
use git;
use tags;
use rustc_serialize::json::{Json, ToJson};

use diecast::Item;
use diecast::support;

// TODO toml re-export?
use metadata;

use super::PublishDate;

#[inline]
fn item_meta(item: &Item) -> Option<&toml::Value> {
    item.extensions.get::<metadata::toml::Metadata>()
}

#[inline]
fn item_title(item: &Item) -> Option<String> {
    item_meta(item)
        .and_then(|m|
            m.lookup("title")
                .and_then(toml::Value::as_str)
                .map(String::from))
}

// have comments on by default unless explicitly disabled
#[inline]
fn item_comments(item: &Item) -> bool {
    item_meta(item)
        .and_then(|m|
            m.lookup("comments")
                .and_then(toml::Value::as_bool))
        .unwrap_or(true)
}

#[inline]
fn item_tags(item: &Item) -> Option<String> {
    item_meta(item)
        .and_then(|m|
            m.lookup("tags")
            .and_then(toml::Value::as_slice)
            .map(|tags|
                tags.iter()
                .map(|tag|
                    if let Some(tag) = tag.as_str() {
                        let url = support::slugify(&tag);
                        format!(r#"<a href="/tags/{}">{}</a>"#, url, tag)
                    } else {
                        String::new()
                    }
                )
                .collect::<Vec<String>>()
                .connect(", ")))
}

#[inline]
fn item_url(item: &Item) -> Option<String> {
    item.route().writing()
        .and_then(Path::parent)
        .and_then(Path::to_str)
        .map(String::from)
}

#[inline]
fn item_path(item: &Item) -> Option<String> {
    item.route().reading()
        .and_then(Path::to_str)
        .map(String::from)
}

#[inline]
fn item_date(item: &Item) -> Option<String> {
    item.extensions.get::<PublishDate>()
        .map(|d| d.format("%B %e, %Y").to_string())
}

#[inline]
fn item_git(item: &Item) -> Option<String> {
    item.extensions.get::<git::Git>()
    .and_then(|git|
        item.source()
        .and_then(|path|
            path.to_str()
            .map(|s| (git, String::from(s)))))
    .map(|(git, path)| {
        let sha = git.sha.to_string().chars().take(7).collect::<String>();
        // TODO
        // detect user/repo by parsing remote?
        let stem = "blaenk/site";
        format!(
"<a href=\"https://github.com/{stem}/commits/master/{path}\">History</a>\
<span class=\"hash\">, \
<a href=\"https://github.com/{stem}/commit/{sha}\" title=\"{message}\">{sha}</a>\
</span>",
        path=path, stem=stem, sha=sha, message=git.message)
    })
}

pub fn with_body(item: &Item) -> Json {
    let mut bt = BTreeMap::new();
    bt.insert(String::from("body"),  item.body.to_json());
    bt.to_json()
}

pub fn append_site(title: Option<String>) -> Option<String> {
    title.map(|t| format!("{} - Blaenk Denum", t))
}

pub fn post_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();

    bt.insert(String::from("partial"), String::from("post").to_json());

    bt.insert(String::from("title"), item_title(&item).to_json());
    bt.insert(String::from("page_title"), append_site(item_title(&item)).to_json());
    bt.insert(String::from("comments"),   item_comments(&item).to_json());
    bt.insert(String::from("url"),   item_url(&item).to_json());
    bt.insert(String::from("path"), item_path(&item).to_json());
    bt.insert(String::from("body"),  item.body.to_json());
    bt.insert(String::from("date"),  item_date(&item).to_json());
    bt.insert(String::from("git"),   item_git(&item).to_json());
    bt.insert(String::from("tags"),   item_tags(&item).to_json());

    Json::Object(bt)
}

pub fn page_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();

    bt.insert(String::from("partial"), String::from("page").to_json());

    bt.insert(String::from("title"), item_title(&item).to_json());
    bt.insert(String::from("page_title"), append_site(item_title(&item)).to_json());
    bt.insert(String::from("url"),   item_url(&item).to_json());
    bt.insert(String::from("comments"),   item_comments(&item).to_json());
    bt.insert(String::from("path"), item_path(&item).to_json());
    bt.insert(String::from("body"),  item.body.to_json());

    Json::Object(bt)
}

pub fn tags_index_template(item: &Item) -> Json {
    // TODO: how to paginate as well??
    let mut bt = BTreeMap::new();
    let mut items = vec![];
    let mut tg = String::new();

    if let Some(tag) = item.extensions.get::<tags::Tag>() {
        for post in tag.items.iter() {
            let mut itm = BTreeMap::new();

            tg = tag.tag.clone();

            itm.insert(String::from("title"), item_title(&post).to_json());
            itm.insert(String::from("url"), item_url(&post).to_json());

            items.push(itm);
        }
    }

    bt.insert(String::from("partial"), String::from("tags").to_json());
    bt.insert(String::from("page_title"), format!("Posts tagged: {}", tg).to_json());
    bt.insert(String::from("tag"), tg.to_json());
    bt.insert(String::from("items"), items.to_json());

    Json::Object(bt)
}

pub fn posts_index_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();
    let mut items = vec![];

    for post in item.bind().dependencies["posts"].items() {
        let mut itm = BTreeMap::new();

        itm.insert(String::from("title"), item_title(post).to_json());
        itm.insert(String::from("url"), item_url(post).to_json());

        items.push(itm);
    }

    bt.insert(String::from("partial"), String::from("index").to_json());
    bt.insert(String::from("page_title"), String::from("Blaenk Denum").to_json());
    bt.insert(String::from("items"), items.to_json());

    Json::Object(bt)
}

pub fn note_item(item: &Item) -> BTreeMap<String, Json> {
    let mut itm = BTreeMap::new();

    itm.insert(String::from("title"), item_title(&item).to_json());
    itm.insert(String::from("comments"),   item_comments(&item).to_json());
    itm.insert(String::from("page_title"), append_site(item_title(&item)).to_json());
    itm.insert(String::from("url"), item_url(&item).to_json());
    itm.insert(String::from("date"), item_date(&item).to_json());
    itm.insert(String::from("git"), item_git(&item).to_json());

    itm
}

// TODO: DRY
// this and the next functions only differ by
// the dependency being captured
pub fn notes_index_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();
    let mut items = vec![];

    for post in item.bind().dependencies["notes"].items() {
        items.push(note_item(post));
    }

    bt.insert(String::from("partial"), String::from("index").to_json());
    bt.insert(String::from("page_title"), String::from("Notes - Blaenk Denum").to_json());
    bt.insert(String::from("items"), items.to_json());

    Json::Object(bt)
}

pub fn work_index_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();
    let mut items = vec![];

    for post in item.bind().dependencies["work"].items() {
        items.push(note_item(post));
    }

    bt.insert(String::from("partial"), String::from("index").to_json());
    bt.insert(String::from("page_title"), String::from("Work - Blaenk Denum").to_json());
    bt.insert(String::from("items"), items.to_json());

    Json::Object(bt)
}
