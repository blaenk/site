#[macro_use]
extern crate diecast;
extern crate diecast_websocket;
extern crate diecast_git;
extern crate diecast_rss;
extern crate diecast_handlebars;
extern crate diecast_scss;

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

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use rustc_serialize::json::{Json, ToJson};
use std::process::{Command, Child};

use time::PreciseTime;
use glob::Pattern as Glob;

use diecast::{
    Configuration,
    Handle,
    Rule,
    Item,
};

use diecast::command;
use diecast::util::route;
use diecast::util::source;
use diecast::util::handle::{Chain, bind, item};

#[derive(Clone)]
pub struct Tag {
    pub tag: String,
    pub items: Arc<Vec<Arc<Item>>>,
}

impl typemap::Key for Tag {
    type Value = Tag;
}

fn slugify(s: &str) -> String {
    s.chars()
    .filter_map(|c| {
        let is_ws = c.is_whitespace();
        if c.is_alphanumeric() || is_ws {
            let c = c.to_lowercase().next().unwrap();
            if is_ws { Some('-') }
            else { Some(c) }
        } else {
            None
        }
    })
    .collect()
}

fn tag_index(bind: Arc<::diecast::bind::Data>) -> Vec<Item> {
    let mut items = vec![];

    if let Some(tags) = bind.dependencies["posts"].data().extensions.read().unwrap().get::<bind::Tags>() {
        for (tag, itms) in tags {
            let url = slugify(&tag);

            let mut pgs = source::pages(itms.len(), 5, &move |page: usize| -> PathBuf {
                if page == 0 {
                    PathBuf::from(&format!("tags/{}/index.html", url))
                } else {
                    PathBuf::from(&format!("tags/{}/{}/index.html", url, page))
                }
            }, bind.clone());

            for item in &mut pgs {
                item.extensions.insert::<Tag>(Tag { tag: tag.clone(), items: itms.clone() });
            }

            items.extend(pgs);
        }
    }

    items
}

fn post_template(item: &Item) -> Json {
    let mut bt = BTreeMap::new();

    // TODO: don't predicate these things on metadata existing
    if let Some(meta) = item.extensions.get::<item::Metadata>() {
        bt.insert(String::from("body"), item.body.to_json());

        if let Some(title) = meta.lookup("title").and_then(toml::Value::as_str).map(ToJson::to_json) {
            bt.insert(String::from("title"), title);
        }

        if let Some(path) = item.route().writing().and_then(Path::parent).and_then(Path::to_str).map(ToJson::to_json) {
            bt.insert(String::from("url"), path);
        }

        if let Some(date) = item.extensions.get::<item::Date>() {
            bt.insert(String::from("date"), date.format("%B %e, %Y").to_string().to_json());
        }

        if let Some(git) = item.extensions.get::<diecast_git::Git>() {
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
                let url = slugify(&tag);
                // TODO: sanitize the tag url
                format!("<a href=\"/tags/{}\">{}</a>", url, tag)
            })
            .collect::<Vec<String>>();
            bt.insert(String::from("tags"), tags.connect(", ").to_json());
        }
    }

    Json::Object(bt)
}

fn posts_index_template(item: &Item) -> Json {
    let page = item.extensions.get::<item::Page>().unwrap();
    let mut bt = BTreeMap::new();
    let mut items = vec![];

    for post in &item.bind().dependencies["posts"].items()[page.range.clone()] {
        let mut itm = BTreeMap::new();

        if let Some(meta) = post.extensions.get::<item::Metadata>() {
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

    if let Some((_, ref path)) = page.prev {
        bt.insert(String::from("prev"), path.parent().unwrap().to_str().unwrap().to_json());
    }

    if let Some((_, ref path)) = page.next {
        bt.insert(String::from("next"), path.parent().unwrap().to_str().unwrap().to_json());
    }

    Json::Object(bt)
}

fn tags_index_template(item: &Item) -> Json {
    // TODO: how to paginate as well??
    let page = item.extensions.get::<item::Page>().unwrap();
    let mut bt = BTreeMap::new();
    let mut items = vec![];
    let mut tg = String::new();

    if let Some(tag) = item.extensions.get::<Tag>() {
        for post in &tag.items[page.range.clone()] {
            let mut itm = BTreeMap::new();

            tg = tag.tag.clone();

            if let Some(meta) = post.extensions.get::<item::Metadata>() {
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

    if let Some((_, ref path)) = page.prev {
        bt.insert(String::from("prev"), path.parent().unwrap().to_str().unwrap().to_json());
    }

    if let Some((_, ref path)) = page.next {
        bt.insert(String::from("next"), path.parent().unwrap().to_str().unwrap().to_json());
    }

    Json::Object(bt)
}

fn notes_index_template(item: &Item) -> Json {
    let page = item.extensions.get::<item::Page>().unwrap();
    let mut bt = BTreeMap::new();
    let mut items = vec![];

    for post in &item.bind().dependencies["notes"].items()[page.range.clone()] {
        let mut itm = BTreeMap::new();

        if let Some(meta) = post.extensions.get::<item::Metadata>() {
            if let Some(title) = meta.lookup("title") {
                itm.insert(String::from("title"), title.as_str().unwrap().to_json());
            }

            if let Some(path) = post.route().writing() {
                itm.insert(String::from("url"), path.parent().unwrap().to_str().unwrap().to_json());
            }

            if let Some(date) = item.extensions.get::<item::Date>() {
                bt.insert(String::from("date"), date.format("%B %e, %Y").to_string().to_json());
            }

            if let Some(git) = item.extensions.get::<diecast_git::Git>() {
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

    if let Some((_, ref path)) = page.prev {
        bt.insert(String::from("prev"), path.parent().unwrap().to_str().unwrap().to_json());
    }

    if let Some((_, ref path)) = page.next {
        bt.insert(String::from("next"), path.parent().unwrap().to_str().unwrap().to_json());
    }

    Json::Object(bt)
}

fn layout_template(item: &Item) -> Json {
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

    let ws_tx = diecast_websocket::init();

    println!("pig server initialized");

    let context = Arc::new(Mutex::new(zmq::Context::new()));

    let templates =
        Rule::read("templates")
        .source(source::select("templates/*.html".parse::<Glob>().unwrap()))
        .handler(Chain::new()
            .link(bind::parallel_each(item::read))
            .link(diecast_handlebars::register_templates));

    let statics =
        Rule::read("statics")
        .source(source::select(or!(
            "images/**/*".parse::<Glob>().unwrap(),
            "static/**/*".parse::<Glob>().unwrap(),
            "js/**/*".parse::<Glob>().unwrap(),
            "favicon.png",
            "CNAME"
        )))
        .handler(bind::parallel_each(Chain::new()
            .link(route::identity)
            .link(item::copy)));

    let scss =
        Rule::read("scss")
        .source(source::select("scss/**/*.scss".parse::<Glob>().unwrap()))
        .handler(diecast_scss::scss("scss/screen.scss", "css/screen.css"));

    let pages =
        Rule::read("pages")
        .depends_on(&templates)
        .source(source::select("pages/*.markdown".parse::<Glob>().unwrap()))
        .handler(Chain::new()
            .link(bind::parallel_each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)))
            // TODO: replace with some sort of filter/only_if
            // .link(bind::retain(item::publishable))
            .link(bind::parallel_each(Chain::new()
                .link(markdown(context.clone()))
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
                .link(diecast_handlebars::render_template(&templates, "page", post_template))
                .link(diecast_handlebars::render_template(&templates, "layout", layout_template))
                .link(item::write))));

    let notes =
        Rule::read("notes")
        .depends_on(&templates)
        .source(source::select("notes/*.markdown".parse::<Glob>().unwrap()))
        .handler(Chain::new()
            .link(bind::parallel_each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)
                .link(item::date)))
            // TODO: replace with some sort of filter/only_if
            // .link(bind::retain(item::publishable))
            .link(bind::parallel_each(Chain::new()
                .link(markdown(context.clone()))
                .link(route::pretty)))
            .link(diecast_websocket::pipe(ws_tx.clone()))
            .link(diecast_git::git)
            .link(bind::parallel_each(Chain::new()
                .link(diecast_handlebars::render_template(&templates, "note", post_template))
                .link(diecast_handlebars::render_template(&templates, "layout", layout_template))
                .link(item::write)))
            .link(bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
                b.cmp(a)
            })));

    let notes_index =
        Rule::create("note index")
        .depends_on(&notes)
        .depends_on(&templates)
        .source(source::paginate(&notes, 5, |page: usize| -> PathBuf {
            if page == 0 {
                PathBuf::from("notes/index.html")
            } else {
                PathBuf::from(&format!("notes/{}/index.html", page))
            }
        }))
        .handler(bind::parallel_each(Chain::new()
            .link(diecast_handlebars::render_template(&templates, "index", notes_index_template))
            .link(diecast_handlebars::render_template(&templates, "layout", layout_template))
            .link(item::write)));

    let posts =
        Rule::read("posts")
        .depends_on(&templates)
        .source(source::select("posts/*.markdown".parse::<Glob>().unwrap()))
        .handler(Chain::new()
            .link(bind::parallel_each(Chain::new()
                .link(item::read)
                .link(item::parse_metadata)
                .link(item::date)))
            .link(bind::retain(item::publishable))
            // TODO need this:
            // .link(bind::cond(item::publishable, handler))
            .link(bind::parallel_each(Chain::new()
                .link(markdown(context.clone()))
                .link(item::save_version("rendered"))
                .link(route::pretty)))
            .link(bind::tags)
            .link(diecast_websocket::pipe(ws_tx))
            .link(diecast_git::git)
            .link(bind::parallel_each(Chain::new()
                .link(diecast_handlebars::render_template(&templates, "post", post_template))
                .link(diecast_handlebars::render_template(&templates, "layout", layout_template))
                .link(item::write)))
            .link(bind::sort_by(|a, b| {
                let a = a.extensions.get::<item::Date>().unwrap();
                let b = b.extensions.get::<item::Date>().unwrap();
                b.cmp(a)
            })));

    let posts_index =
        Rule::create("post index")
        .depends_on(&posts)
        .depends_on(&templates)
        .source(source::paginate(&posts, 5, |page: usize| -> PathBuf {
            if page == 0 {
                PathBuf::from("index.html")
            } else {
                PathBuf::from(&format!("{}/index.html", page))
            }
        }))
        .handler(bind::parallel_each(Chain::new()
            .link(diecast_handlebars::render_template(&templates, "index", posts_index_template))
            .link(diecast_handlebars::render_template(&templates, "layout", layout_template))
            .link(item::write)));

    // TODO: this should be expressed in such a way that it is possible to paginate
    let tags =
        Rule::create("tag index")
        .depends_on(&templates)
        .depends_on(&posts)
        .source(tag_index)
        .handler(bind::parallel_each(Chain::new()
            .link(diecast_handlebars::render_template(&templates, "tags", tags_index_template))
            .link(diecast_handlebars::render_template(&templates, "layout", layout_template))
            .link(item::write)));

    let feed =
        Rule::create("feed")
        .depends_on(&posts)
        .source(source::create("rss.xml"))
        .handler(bind::each(Chain::new()
            .link(diecast_rss::rss)
            .link(item::write)));

    let not_found =
        Rule::create("404")
        .depends_on(&templates)
        .source(source::create("404.html"))
        .handler(bind::each(Chain::new()
            .link(diecast_handlebars::render_template(&templates, "404", |_| Json::Null))
            .link(diecast_handlebars::render_template(&templates, "layout", layout_template))
            .link(item::write)));

    let config = Configuration::new();
    let mut command = command::from_args(config);

    command.site().register(templates);
    command.site().register(statics);
    command.site().register(scss);
    command.site().register(pages);
    command.site().register(posts);
    command.site().register(posts_index);
    command.site().register(tags);
    command.site().register(notes);
    command.site().register(notes_index);
    command.site().register(feed);
    command.site().register(not_found);

    let start = PreciseTime::now();

    command.run();

    let end = PreciseTime::now();

    println!("time elapsed: {}", start.to(end));

    // FIXME: main thread doesn't wait for children?
    println!("EXITING");

    pig_handle.kill().unwrap();
}

pub fn markdown(context: Arc<Mutex<zmq::Context>>) -> Markdown {
    Markdown { context: context }
}

pub struct Markdown {
    context: Arc<Mutex<zmq::Context>>,
}

impl Handle<Item> for Markdown {
    fn handle(&self, item: &mut Item) -> diecast::Result {
        use std::collections::HashMap;
        use regex::{Regex, Captures};
        use hoedown::Render;
        use hoedown::renderer::html;

        let pattern = Regex::new(r"(?m)^\*\[(?P<abbr>.+)\]: (?P<full>.+)$").unwrap();
        let mut abbrs = HashMap::new();

        let clean = pattern.replace_all(&item.body, |caps: &Captures| -> String {
            let abbr = String::from(caps.name("abbr").unwrap());
            let full = String::from(caps.name("full").unwrap());

            assert!(
                !abbr.chars().any(|c| c == '|'),
                "abbreviations shouldn't contain the '|' character!");

            abbrs.insert(abbr, full);
            String::new()
        });

        trace!("collected abbreviations");

        let meta = item.extensions.get::<item::Metadata>();

        if let Some(meta) = meta {
            if !meta.lookup("toc.show").and_then(toml::Value::as_bool).unwrap_or(false) {
                // TODO: tell render not to generate toc
            }
        }

        // if there is metadata, parse the field
        // otherwise assume left align
        let align =
            meta.and_then(|m|
                m.lookup("toc.align")
                .and_then(toml::Value::as_str)
                .map(|align| {
                    match align {
                        "left" => renderer::Align::Left,
                        "right" => renderer::Align::Right,
                        _ => panic!("invalid value for toc.align. either `left` or `right`"),
                    }
                }))
            .unwrap_or(renderer::Align::Left);

        trace!("got toc alignment");

        let document =
            hoedown::Markdown::new(&clean)
            .extensions({
                use hoedown::*;

                AUTOLINK |
                FENCED_CODE |
                FOOTNOTES |
                MATH |
                MATH_EXPLICIT |
                SPACE_HEADERS |
                STRIKETHROUGH |
                SUPERSCRIPT |
                TABLES
            });

        let enabled =
            meta.and_then(|m| m.lookup("toc.show").and_then(toml::Value::as_bool))
            .unwrap_or(false);

        let mut renderer = self::renderer::Renderer::new(abbrs, align, enabled, self.context.clone());

        trace!("constructed renderer");

        let buffer = renderer.render(&document);

        trace!("rendered markdown");

        let pattern = Regex::new(r"<p>::toc::</p>").unwrap();

        let mut smartypants = hoedown::Buffer::new(64);
        html::smartypants(&buffer, &mut smartypants);

        trace!("smartypants");

        item.body = pattern.replace(&smartypants.to_str().unwrap(), &renderer.toc[..]);

        trace!("inserted toc");

        Ok(())
    }
}

mod renderer {
    use hoedown::{Buffer, Render, Wrapper, Markdown};
    use hoedown::renderer;
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use regex::Regex;
    use zmq;

    pub enum Align {
        Left,
        Right,
    }

    pub struct Pass;
    impl Render for Pass {
        fn link(&mut self, output: &mut Buffer, content: &Buffer, _link: &Buffer, _title: &Buffer) -> bool {
            output.pipe(content);
            true
        }
    }

    fn sanitize(content: &str) -> String {
        let doc =
            Markdown::new(content)
            .extensions({
                use hoedown::*;

                AUTOLINK |
                FENCED_CODE |
                FOOTNOTES |
                MATH |
                MATH_EXPLICIT |
                SPACE_HEADERS |
                STRIKETHROUGH |
                SUPERSCRIPT |
                TABLES
            });

        let output = String::from(Pass.render_inline(&doc).to_str().unwrap());

        output.chars()
        .filter(|&c|
            c.is_alphabetic() || c.is_digit(10) ||
            c == '_' || c == '-' || c == '.' || c == ' '
        )
        .map(|c| {
            let c = c.to_lowercase().next().unwrap();

            if c.is_whitespace() { '-' }
            else { c }
        })
        .skip_while(|c| !c.is_alphabetic())
        .collect()
    }

    pub struct Renderer {
        pub html: renderer::html::Html,
        abbreviations: HashMap<String, String>,
        matcher: Regex,

        is_toc_enabled: bool,

        pub toc: String,

        /// the current header level
        toc_level: i32,

        /// the offset of the first header sighted from 0
        toc_offset: i32,

        toc_align: Align,

        socket: zmq::Socket,
    }

    impl Renderer {
        pub fn new(abbrs: HashMap<String, String>, align: Align, enabled: bool, context: Arc<Mutex<zmq::Context>>) -> Renderer {
            let joined: String =
                abbrs.keys().cloned().collect::<Vec<String>>().connect("|");

            // TODO: shouldn't have | in abbr
            let matcher = Regex::new(&joined).unwrap();

            let mut socket = context.lock().unwrap().socket(zmq::REQ).unwrap();
            socket.connect("tcp://127.0.0.1:5555").unwrap();

            Renderer {
                html: renderer::html::Html::new(renderer::html::Flags::empty(), 0),
                abbreviations: abbrs,
                matcher: matcher,

                is_toc_enabled: enabled,
                toc: String::new(),
                toc_level: 0,
                toc_offset: 0,
                toc_align: align,

                socket: socket,
            }
        }
    }

    #[allow(unused_variables)]
    impl Wrapper for Renderer {
        type Base = renderer::html::Html;

        fn base(&mut self) -> &mut renderer::html::Html {
            &mut self.html
        }

        fn code_block(&mut self, output: &mut Buffer, code: &Buffer, lang: &Buffer) {
            use std::io::Write;

            let lang = if lang.is_empty() {
                "text"
            } else {
                lang.to_str().unwrap()
            };

            write!(output,
r#"<figure class="codeblock">
<pre>
<code class="highlight language-{}">"#, lang).unwrap();

            if lang == "text" {
                output.pipe(code);
            } else {
                let lang = zmq::Message::from_slice(lang.as_bytes()).unwrap();
                self.socket.send_msg(lang, zmq::SNDMORE).unwrap();

                let code = zmq::Message::from_slice(&code).unwrap();
                self.socket.send_msg(code, 0).unwrap();

                let highlighted = self.socket.recv_msg(0).unwrap();

                output.write(&highlighted).unwrap();
            }

            output.write(b"</code></pre></figure>").unwrap();
        }

        fn normal_text(&mut self, output: &mut Buffer, text: &Buffer) {
            use regex::Captures;
            use std::io::Write;

            if self.abbreviations.is_empty() {
                output.pipe(text);
                return;
            }

            // replace abbreviations with their full form
            let replaced = self.matcher.replace_all(text.to_str().unwrap(), |caps: &Captures| -> String {
                let abbr = caps.at(0).unwrap();
                let full = self.abbreviations.get(abbr).unwrap().clone();

                format!(r#"<abbr title="{}">{}</abbr>"#, full, abbr)
            });

            output.write(replaced.as_bytes()).unwrap();
        }

        fn after_render(&mut self, output: &mut Buffer, inline_render: bool) {
            if inline_render || !self.is_toc_enabled {
                return;
            }

            while self.toc_level > 0 {
                self.toc.push_str("</li>\n</ol>\n");
                self.toc_level -= 1;
            }

            self.toc.push_str("</nav>");
        }

        fn header(&mut self, output: &mut Buffer, content: &Buffer, level: i32) {
            use std::io::Write;

            if !self.is_toc_enabled {
                return;
            }

            // first header sighted
            if self.toc_level == 0 {
                self.toc_offset = level - 1;

                self.toc.push_str(r#"<nav id="toc""#);

                if let Align::Right = self.toc_align {
                    self.toc.push_str(r#"class="right-toc""#)
                }

                self.toc.push_str(">\n<h3>Contents</h3>");
            }

            let level = level - self.toc_offset;

            if level > self.toc_level {
                while level > self.toc_level {
                    self.toc.push_str("<ol>\n<li>\n");
                    self.toc_level += 1;
                }
            } else if level < self.toc_level {
                self.toc.push_str("</li>\n");

                while level < self.toc_level {
                    self.toc.push_str("</ol>\n</li>\n");
                    self.toc_level -= 1;
                }

                self.toc.push_str("<li>\n");
            } else {
                self.toc.push_str("</li>\n<li>\n");
            }

            let sanitized = sanitize(content.to_str().unwrap());
            self.toc.push_str(r##"<a href="#"##);
            self.toc.push_str(&sanitized);
            self.toc.push_str(r#"">"#);

            let bytes: &[u8] = content.as_ref();

            let doc =
                Markdown::from(bytes)
                .extensions({
                    use hoedown::*;

                    AUTOLINK |
                    FENCED_CODE |
                    FOOTNOTES |
                    MATH |
                    MATH_EXPLICIT |
                    SPACE_HEADERS |
                    STRIKETHROUGH |
                    SUPERSCRIPT |
                    TABLES
                });

            let rendered = self.html.render_inline(&doc);

            self.toc.push_str(rendered.to_str().unwrap());
            self.toc.push_str("</a>\n");

            write!(output,
r##"<h2 id="{}">
<span class="hash">#</span>
<a href="#{}" class="header-link">{}</a>
</h2>"##, sanitized, sanitized, content.to_str().unwrap()).unwrap();
        }
    }

    wrap!(Renderer);
}

