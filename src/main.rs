#[macro_use]
extern crate diecast;
extern crate diecast_live as live;
extern crate diecast_websocket as websocket;
extern crate diecast_git as git;
extern crate diecast_feed as feed;
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
extern crate zmq;
extern crate sha1;
extern crate syncbox;

use std::process::{Command, Child};
use std::sync::{Arc, Mutex};

use time::PreciseTime;
use syncbox::ThreadPool;

use diecast::{Site, Rule, Item};
use diecast::command;
use diecast::util::route;
use diecast::util::handle::{handle_if, bind, item};

// item: read, write, copy
// bind: each, retain, sort_by

mod markdown;
mod view;
mod helpers;

pub struct PublishDate;

impl typemap::Key for PublishDate {
    type Value = chrono::NaiveDate;
}

fn github_pages_deploy(remote: &'static str, branch: &'static str,
                       git: &'static str, target: &'static str) {
    use std::process::Command;
    use std::env;
    use diecast;

    // has the repo been initialized?
    let initialized =
        diecast::support::file_exists(git);

    if !initialized {
        println!("  [*] initializing repository");
        // git init --separate-git-dir .deploy.git
        Command::new("git")
            .arg("init").arg("--bare").arg(git)
            .status()
            .unwrap_or_else(|e|
                panic!("git init failed: {}", e));
    }

    // git rev-parse --short HEAD
    let out =
    Command::new("git")
        .arg("rev-parse").arg("--short").arg("HEAD")
        .output()
        .unwrap_or_else(|e|
            panic!("git rev-parse failed: {}", e));

    let sha = String::from_utf8_lossy(&out.stdout).into_owned();
    let short = sha.trim_right();

    env::set_var("GIT_DIR", git);
    env::set_var("GIT_WORK_TREE", target);

    if !initialized {
        println!("  [*] setting remote");
        // git remote add upstream <remote>
        Command::new("git")
            .arg("remote").arg("add").arg("upstream")
            .arg(remote)
            .status()
            .unwrap_or_else(|e|
                panic!("git init failed: {}", e));

        println!("  [*] fetching remote");
        // git fetch upstream
        Command::new("git")
            .arg("fetch").arg("upstream")
            .status()
            .unwrap_or_else(|e|
                panic!("git init failed: {}", e));

        println!("  [*] resetting to {}", branch);
        // git reset upstream/master
        Command::new("git")
            .arg("reset").arg(format!("upstream/{}", branch))
            .status()
            .unwrap_or_else(|e|
                panic!("git init failed: {}", e));
    }

    println!("  [*] staging all files");
    // git add --all .
    Command::new("git")
        .arg("add").arg("--all").arg(".")
        .status()
        .unwrap_or_else(|e|
            panic!("git init failed: {}", e));

    println!("  [*] deploying site generated from {}", short);
    // git commit -m "generated from <sha>"
    Command::new("git")
        .arg("commit").arg("-m").arg(format!("generated from {}", short))
        .status()
        .unwrap_or_else(|e|
            panic!("git init failed: {}", e));

    println!("  [*] pushing");
    // git push upstream HEAD:master -f
    Command::new("git")
        .arg("push").arg("upstream").arg("master").arg("-f")
        .status()
        .unwrap_or_else(|e|
            panic!("git init failed: {}", e));

    env::remove_var("GIT_DIR");
    env::remove_var("GIT_WORK_TREE");

    println!("  [*] deploy complete");
}

fn pig() -> Child {
    println!("initializing pig server...");

    Command::new("python")
        .arg("scripts/pig.py")
        .spawn()
        .unwrap()
}

fn is_pushable(item: &Item) -> bool {
    item.extensions.get::<metadata::toml::Metadata>()
    .and_then(|m| m.lookup("push").and_then(toml::Value::as_bool))
    .unwrap_or(true)
}

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

    let mut pig_handle = pig();

    // initialize_pool!(4).unwrap();

    let context = Arc::new(Mutex::new(zmq::Context::new()));

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

    let pool = bind::PooledEach::new(ThreadPool::fixed_size(2));

    let templates =
        Rule::named("templates")
        .handler(chain![
            bind::select(glob!("templates/*.html")),
            pool.each(item::read),
            handlebars::register_templates])
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
                "CNAME")),
            pool.each(chain![
                route::identity,
                item::copy])])
        .build();

    let scss =
        Rule::named("scss")
        .handler(chain![
            bind::select(glob!("scss/**/*.scss")),
            scss::scss("scss/screen.scss", "css/screen.css")])
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
                markdown::markdown(context.clone()),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                // TODO
                // this isn't even being used
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
            git::git,
            pool.each(chain![
                handlebars::render(&templates, "layout", view::post_template),
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
            pool.each(chain![
                handlebars::render(&templates, "layout", view::posts_index_template),
                item::write])])
        .build();

    let pages =
        Rule::named("pages")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("pages/*.markdown")),
            pool.each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                markdown::markdown(context.clone()),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty_page]),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::page_template),
                item::write])])
        .build();

    let notes =
        Rule::named("notes")
        .depends_on(&templates)
        .handler(chain![
            bind::select(glob!("notes/*.markdown")),
            pool.each(chain![
                item::read,
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                markdown::markdown(context.clone()),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty]),
            git::git,
            pool.each(chain![
                handlebars::render(&templates, "layout", view::post_template),
                item::write]),
            bind::sort_by(|a, b| {
                let a = a.extensions.get::<PublishDate>().unwrap();
                let b = b.extensions.get::<PublishDate>().unwrap();
                b.cmp(a)
            })])
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
                metadata::toml::parse]),
            bind::retain(helpers::publishable),
            pool.each(chain![
                helpers::set_date,
                markdown::markdown(context.clone()),
                handle_if(is_pushable, websocket::pipe(ws_tx.clone())),
                route::pretty]),
            git::git,
            pool.each(chain![
                handlebars::render(&templates, "layout", view::post_template),
                item::write]),
            bind::sort_by(|a, b| {
                let a = a.extensions.get::<metadata::toml::Metadata>().unwrap();
                let b = b.extensions.get::<metadata::toml::Metadata>().unwrap();
                let a_title = a.lookup("title").unwrap().as_str().unwrap();
                let b_title = b.lookup("title").unwrap().as_str().unwrap();
                a_title.cmp(b_title)
            })])
        .build();

    let notes_index =
        Rule::named("note index")
        .depends_on(&notes)
        .depends_on(&templates)
        .handler(chain![
            bind::create("notes/index.html"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::notes_index_template),
                item::write])])
        .build();

    let work_index =
        Rule::named("work index")
        .depends_on(&work)
        .depends_on(&templates)
        .handler(chain![
            bind::create("work/index.html"),
            pool.each(chain![
                handlebars::render(&templates, "layout", view::work_index_template),
                item::write])])
        .build();

    let tags =
        Rule::named("tag index")
        .depends_on(&templates)
        .depends_on(&posts)
        .handler(chain![
            helpers::tag_index,
            pool.each(chain![
                handlebars::render(&templates, "layout", view::tags_index_template),
                item::write])])
        .build();

    let feed =
        Rule::named("feed")
        .depends_on(&posts)
        .handler(chain![
            feed::rss::create("rss.xml",
                "Blaenk Denum",
                "http://www.blaenkdenum.com",
                helpers::rss_handler),
            pool.each(item::write)])
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
                item::write])])
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

    let mut site = Site::new(rules);

    let command =
        command::Builder::new()
        .command("live", live::Live::new("0.0.0.0:4000"))
        // TODO
        // if config.output dir doesn't exist, site.build()?
        .command("deploy",
                 command::deploy::Deploy::new(|_: &Site| -> diecast::Result<()> {
                     github_pages_deploy(
                         "git@github.com:blaenk/blaenk.github.io.git", "master",
                         ".deploy.git", "output");

                     Ok(())
                 }))
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

    pig_handle.kill().unwrap();
}
