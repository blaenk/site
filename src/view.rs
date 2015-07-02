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

pub fn post_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();

    // TODO: don't predicate these things on metadata existing
    if let Some(meta) = item.extensions.get::<metadata::toml::Metadata>() {
        bt.insert(String::from("body"), item.body.to_json());

        if let Some(title) = meta.lookup("title").and_then(toml::Value::as_str).map(ToJson::to_json) {
            bt.insert(String::from("title"), title);
        }

        if let Some(path) = item.route().writing().and_then(Path::parent).and_then(Path::to_str).map(ToJson::to_json) {
            bt.insert(String::from("url"), path);
        }

        if let Some(date) = item.extensions.get::<PublishDate>() {
            bt.insert(String::from("date"), date.format("%B %e, %Y").to_string().to_json());
        }

        if let Some(git) = item.extensions.get::<git::Git>() {
            let sha = git.sha.to_string().chars().take(7).collect::<String>();
            let path = item.source().unwrap();

            // TODO: change the url and branch when ready
            let res =
                format!(
"<a href=\"https://github.com/blaenk/diecast/commits/master/{}\">History</a>\
<span class=\"hash\">, \
<a href=\"https://github.com/blaenk/diecast/commit/{}\" title=\"{}\">{}</a>\
</span>",
                path.to_str().unwrap(), sha, git.message, sha);

            bt.insert(String::from("git"), res.to_json());
        }

        if let Some(tags) = meta.lookup("tags").and_then(toml::Value::as_slice) {
            let tags = tags.iter().map(|t| {
                let tag = t.as_str().unwrap();
                let url = support::slugify(&tag);
                // TODO: sanitize the tag url
                format!("<a href=\"/tags/{}\">{}</a>", url, tag)
            })
            .collect::<Vec<String>>();
            bt.insert(String::from("tags"), tags.connect(", ").to_json());
        }
    }

    Json::Object(bt)
}

pub fn posts_index_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();
    let mut items = vec![];

    for post in item.bind().dependencies["posts"].items() {
        let mut itm = BTreeMap::new();

        if let Some(meta) = post.extensions.get::<metadata::toml::Metadata>() {
            if let Some(title) = meta.lookup("title") {
                itm.insert(String::from("title"), title.as_str().unwrap().to_json());
            }

            if let Some(path) = post.route().writing() {
                itm.insert(String::from("url"), path.parent().unwrap().to_str().unwrap().to_json());
            }
        }

        items.push(itm);
    }

    bt.insert(String::from("items"), items.to_json());

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

            if let Some(meta) = post.extensions.get::<metadata::toml::Metadata>() {
                if let Some(title) = meta.lookup("title") {
                    itm.insert(String::from("title"), title.as_str().unwrap().to_json());
                }
            }

            if let Some(path) = post.route().writing() {
                itm.insert(String::from("url"), path.parent().unwrap().to_str().unwrap().to_json());
            }

            items.push(itm);
        }
    }

    bt.insert(String::from("tag"), tg.to_json());
    bt.insert(String::from("items"), items.to_json());

    Json::Object(bt)
}

pub fn notes_index_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();
    let mut items = vec![];

    for post in item.bind().dependencies["notes"].items() {
        let mut itm = BTreeMap::new();

        if let Some(meta) = post.extensions.get::<metadata::toml::Metadata>() {
            if let Some(title) = meta.lookup("title") {
                itm.insert(String::from("title"), title.as_str().unwrap().to_json());
            }

            if let Some(path) = post.route().writing() {
                itm.insert(String::from("url"), path.parent().unwrap().to_str().unwrap().to_json());
            }

            if let Some(date) = item.extensions.get::<PublishDate>() {
                bt.insert(String::from("date"), date.format("%B %e, %Y").to_string().to_json());
            }

            if let Some(git) = item.extensions.get::<git::Git>() {
                let sha = git.sha.to_string().chars().take(7).collect::<String>();
                let path = item.source().unwrap();

                // TODO: change the url and branch when ready
                let res =
                    format!(
    "<a href=\"https://github.com/blaenk/diecast/commits/master/{}\">History</a>\
    <span class=\"hash\">, \
    <a href=\"https://github.com/blaenk/diecast/commit/{}\" title=\"{}\">{}</a>\
    </span>",
                    path.to_str().unwrap(), sha, git.message, sha);

                bt.insert(String::from("git"), res.to_json());
            }
        }

        items.push(itm);
    }

    bt.insert(String::from("items"), items.to_json());

    Json::Object(bt)
}

pub fn layout_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();

    bt.insert(String::from("body"), item.body.to_json());

    // this should probably go in post template handler
    // move partial load to post template
    if let Some(path) = item.route().reading() {
        bt.insert(String::from("path"), path.to_str().unwrap().to_json());
    }

    if let Some(path) = item.route().writing() {
        bt.insert(String::from("url"), format!("{}/", path.parent().unwrap().to_str().unwrap()).to_json());
    }

    Json::Object(bt)
}


