use std::sync::{Arc, Mutex};
use std::process::{Command, Child};

use hoedown;
use zmq;

use diecast::{self, Handle, Item};

pub struct ChildGuard {
    child: Child
}

impl ChildGuard {
    fn new(child: Child) -> ChildGuard {
        ChildGuard { child: child }
    }
}

impl Drop for ChildGuard {
    fn drop(&mut self) {
        match self.child.kill() {
            Err(e) => println!("Couldn't kill child process: {}", e),
            Ok(_) => info!("Successfully killed the child process"),
        }
    }
}

pub fn markdown() -> Markdown {
    let child = Command::new("python")
        .arg("scripts/pig.py")
        .arg("5555")
        .spawn()
        .unwrap();

    Markdown {
        context: Arc::new(Mutex::new(zmq::Context::new().unwrap())),
        pig: Arc::new(Mutex::new(ChildGuard::new(child))),
    }
}

#[derive(Clone)]
pub struct Markdown {
    context: Arc<Mutex<zmq::Context>>,
    pig: Arc<Mutex<ChildGuard>>,
}

impl Handle<Item> for Markdown {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        use regex::Regex;
        use sha1;

        let rendered = {
            let mut hash = sha1::Sha1::new();
            hash.update(item.body.as_bytes());

            let pattern = Regex::new(r"(?m)^\*\[(?P<abbr>[^]]+)\]: (?P<full>.+)$").unwrap();

            let clean = pattern.replace_all(&item.body, "");

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

            let toc_enabled = clean.contains("<toc");

            let mut renderer =
                self::renderer::Renderer::new(toc_enabled, self.context.clone());

            let buf = document.render(&mut renderer);

            let pattern = Regex::new(r"<p><toc[^>]*/></p>").unwrap();

            pattern.replace(&buf.to_str().unwrap(), &renderer.toc[..]).into_owned()
        };

        item.body = rendered;

        Ok(())
    }
}

mod renderer {
    use std::sync::{Arc, Mutex};
    use std::io::Write;
    use hoedown::{Buffer, Render, Wrapper, Markdown};
    use hoedown::renderer;
    use zmq;

    pub enum Align {
        Left,
        Right,
    }

    pub struct Pass;
    impl Render for Pass {
        fn link(&mut self, output: &mut Buffer, content: Option<&Buffer>, _link: Option<&Buffer>, _title: Option<&Buffer>) -> bool {
            content.map(|c| output.write(c));
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

        doc.render_inline(&mut Pass).to_str().unwrap()
            .chars()
            .filter_map(|c| {
                if c.is_whitespace() {
                    return Some('-');
                }

                let is_sluggish =
                    c.is_alphabetic() || c.is_digit(10) || c == '_' || c == '-' || c == '.';

                if is_sluggish {
                    c.to_lowercase().next()
                } else {
                    None
                }
            })
            .skip_while(|c| !c.is_alphabetic())
            .collect()
    }

    pub struct Renderer {
        pub html: renderer::html::Html,

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
        pub fn new(enabled: bool, context: Arc<Mutex<zmq::Context>>) -> Renderer {
            let socket = {
                let mut s = context.lock().unwrap().socket(zmq::REQ).unwrap();
                s.connect("tcp://127.0.0.1:5555").unwrap();
                s
            };

            Renderer {
                html: renderer::html::Html::new(renderer::html::Flags::empty(), 0),

                is_toc_enabled: enabled,
                toc: String::new(),
                toc_level: 0,
                toc_offset: 0,
                toc_align: Align::Left,

                socket: socket,
            }
        }
    }

    wrap!(Renderer);

    #[allow(unused_variables)]
    impl Wrapper for Renderer {
        type Base = renderer::html::Html;

        fn base(&mut self) -> &mut renderer::html::Html {
            &mut self.html
        }

        fn footnote_definition(&mut self, output: &mut Buffer, content: Option<&Buffer>, num: u32) {
            if content.is_none() { return; }

            let content = content.unwrap();

            output.write(format!("\n<li id=\"fn{}\">\n", num).as_bytes()).unwrap();
            let end = content.len() - 5;
            output.write(&content[.. end as usize]).unwrap();
            output.write(format!("&nbsp;<a href=\"#fnref{}\" title=\"continue reading\" rev=\"footnote\"><i class=\"fa fa-level-up\"></i></a></p></li>\n", num).as_bytes()).unwrap();
        }

        fn math(&mut self, output: &mut Buffer, text: Option<&Buffer>, displaymode: i32) -> bool {
            if text.is_none() { return true; }

            let text = text.unwrap();

            if displaymode != 0 {
                output.write(b"<script type=\"math/tex; mode=display\">").unwrap();
            } else {
                output.write(b"<script type=\"math/tex\">").unwrap();
            }

            output.write(&text).unwrap();
            output.write(b"</script>").unwrap();

            true
        }

        fn html_span(&mut self, output: &mut Buffer, text: Option<&Buffer>) -> bool {
            if text.is_none() { return true; }

            let text = text.unwrap();

            let s = text.to_str().unwrap();

            // this is deliberately very naive to avoid the
            // perf-cost of a more strict parse, since I have
            // no need for it and I'm going to be the only one
            // using this
            //
            // toc_align is already left by default, so only change
            // if it's set to right
            if s.starts_with("<toc") && s.contains("right") {
                self.toc_align = Align::Right;
            }

            output.write(text).unwrap();

            true
        }

        fn code_block(&mut self, output: &mut Buffer, code: Option<&Buffer>, lang: Option<&Buffer>) {
            if code.is_none() { return; }

            let code = code.unwrap();
            let lang = String::from(lang.map_or("text", |l| {
                if l.is_empty() {
                    "text"
                } else {
                    l.to_str().unwrap_or("text")
                }
            }));

            use std::io::Write;

            write!(output,
r#"<figure class="codeblock">
<pre>
<code class="highlight language-{}">"#, lang).unwrap();

            if lang == "text" {
                output.write(code).unwrap();
            } else {
                use sha1;
                use std::fs::File;
                use std::io::{Read, Write};
                use diecast;

                // check cache
                let mut hash = sha1::Sha1::new();
                hash.update(lang.as_bytes());
                hash.update(code);

                let digest = hash.digest().to_string();

                let cache = format!("cache/pygments/{}", digest);
                diecast::support::mkdir_p("cache/pygments/").unwrap();

                match File::open(&cache) {
                    Ok(mut f) => {
                        info!("[PYGMENTS] cache hit {}", digest);

                        let mut contents = vec![];
                        f.read_to_end(&mut contents).unwrap();
                        output.write(&contents).unwrap();
                    },
                    Err(e) => {
                        if let ::std::io::ErrorKind::NotFound = e.kind() {
                            info!("[PYGMENTS] cache miss {}", digest);

                            let lang = zmq::Message::from_slice(lang.as_bytes()).unwrap();
                            self.socket.send_msg(lang, zmq::SNDMORE).unwrap();

                            let code = zmq::Message::from_slice(&code).unwrap();
                            self.socket.send_msg(code, 0).unwrap();

                            let highlighted = self.socket.recv_msg(0).unwrap();

                            output.write(&highlighted).unwrap();

                            let mut f = File::create(&cache).unwrap();
                            f.write_all(&highlighted).unwrap();

                            info!("[PYGMENTS] wrote cache {}", digest)
                        } else {
                            error!("[PYGMENTS] SOME ERROR");
                        }
                    },
                }
            }

            output.write(b"</code></pre></figure>").unwrap();
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

        fn header(&mut self, output: &mut Buffer, content: Option<&Buffer>, level: i32) {
            if content.is_none() { return; }

            let content = content.unwrap();

            use std::io::Write;

            if self.is_toc_enabled {
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
            }

            let sanitized = sanitize(content.to_str().unwrap());

            if self.is_toc_enabled {
                self.toc.push_str(r##"<a href="#"##);
                self.toc.push_str(&sanitized);
                self.toc.push_str(r#"">"#);
                self.toc.push_str(content.to_str().unwrap());
                self.toc.push_str("</a>\n");
            }

            write!(output,
r##"<h{level} id="{id}">
<span class="hash">#</span>
<a href="#{id}" class="header-link">{content}</a>
</h{level}>"##, level=level, id=sanitized, content=content.to_str().unwrap()).unwrap();
        }
    }
}
