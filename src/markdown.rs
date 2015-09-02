use std::sync::{Arc, Mutex};

use hoedown;
use zmq;

use diecast::{self, Handle, Item};

pub fn markdown(context: Arc<Mutex<zmq::Context>>) -> Markdown {
    Markdown { context: context }
}

pub struct Markdown {
    context: Arc<Mutex<zmq::Context>>,
}

impl Handle<Item> for Markdown {
    fn handle(&self, item: &mut Item) -> diecast::Result<()> {
        use std::collections::HashMap;
        use std::io::Read;
        use regex::{Regex, Captures};
        use hoedown::Render;
        use sha1;

        let mut hash = sha1::Sha1::new();
        hash.update(item.body.as_bytes());

        let pattern = Regex::new(r"(?m)^\*\[(?P<abbr>[^]]+)\]: (?P<full>.+)$").unwrap();
        let mut abbrs = HashMap::new();

        let clean = pattern.replace_all(&item.body, |caps: &Captures| -> String {
            let abbr = String::from(caps.name("abbr").unwrap());
            let full = String::from(caps.name("full").unwrap());

            assert!(!abbr.chars().any(|c| c == '|'),
                "abbreviations shouldn't contain the '|' character!");

            abbrs.insert(abbr, full);
            String::new()
        });

        trace!("collected abbreviations");

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

        let enabled = clean.contains("<toc");

        let mut renderer =
            self::renderer::Renderer::new(abbrs, enabled, self.context.clone());

        trace!("constructed renderer");

        let buf = renderer.render(&document);

        let pattern = Regex::new(r"<p><toc[^>]*/></p>").unwrap();

        let rendered =
            pattern.replace(&buf.to_str().unwrap(), &renderer.toc[..]);

        item.body = rendered;

        Ok(())
    }
}

mod renderer {
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};
    use std::io::Write;
    use hoedown::{Buffer, Render, Wrapper, Markdown};
    use hoedown::renderer;
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
        pub fn new(abbrs: HashMap<String, String>, enabled: bool, context: Arc<Mutex<zmq::Context>>) -> Renderer {
            let joined: String =
                abbrs.keys().cloned().collect::<Vec<String>>().connect("|");

            // TODO: shouldn't have | in abbr
            let matcher = Regex::new(&joined).unwrap();

            let socket = {
                let mut s = context.lock().unwrap().socket(zmq::REQ).unwrap();
                s.connect("tcp://127.0.0.1:5555").unwrap();
                s
            };

            Renderer {
                html: renderer::html::Html::new(renderer::html::Flags::empty(), 0),
                abbreviations: abbrs,
                matcher: matcher,

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

        fn math(&mut self, output: &mut Buffer, text: &Buffer, displaymode: i32) -> bool {
            if displaymode != 0 {
                output.write(b"<script type=\"math/tex; mode=display\">").unwrap();
            } else {
                output.write(b"<script type=\"math/tex\">").unwrap();
            }

            output.write(&text).unwrap();
            output.write(b"</script>").unwrap();

            true
        }

        fn html_span(&mut self, output: &mut Buffer, text: &Buffer) -> bool {
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

            output.pipe(text);

            true
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
                use sha1;
                use std::fs::File;
                use std::io::{Read, Write};
                use diecast;

                // check cache
                let mut hash = sha1::Sha1::new();
                hash.update(lang.as_bytes());
                hash.update(code);

                let digest = hash.hexdigest();

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

        fn normal_text(&mut self, output: &mut Buffer, text: &Buffer) {
            use regex::Captures;

            if self.abbreviations.is_empty() {
                output.pipe(&text);
                return;
            }

            // replace abbreviations with their full form
            let replaced = self.matcher.replace_all(text.to_str().unwrap(), |caps: &Captures| -> String {
                let abbr = caps.at(0).unwrap();
                let full = self.abbreviations.get(abbr).unwrap().clone();

                format!(r#"<abbr title="{}">{}</abbr>"#, full, abbr)
            });

            let input = Buffer::from(&replaced[..]);
            output.pipe(&input);
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
