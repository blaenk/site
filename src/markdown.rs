use std::sync::{Arc, Mutex};

use zmq;
use toml;
use hoedown;

use diecast::{self, Handle, Item};
use diecast::util::handle::item;

pub fn markdown() -> Markdown {
    Markdown
}

pub struct Markdown;

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

        let mut renderer = self::renderer::Renderer::new(abbrs, align, enabled);

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
    }

    impl Renderer {
        pub fn new(abbrs: HashMap<String, String>, align: Align, enabled: bool) -> Renderer {
            let joined: String =
                abbrs.keys().cloned().collect::<Vec<String>>().connect("|");

            // TODO: shouldn't have | in abbr
            let matcher = Regex::new(&joined).unwrap();

            Renderer {
                html: renderer::html::Html::new(renderer::html::Flags::empty(), 0),
                abbreviations: abbrs,
                matcher: matcher,

                is_toc_enabled: enabled,
                toc: String::new(),
                toc_level: 0,
                toc_offset: 0,
                toc_align: align,
            }
        }
    }

    #[allow(unused_variables)]
    impl Wrapper for Renderer {
        type Base = renderer::html::Html;

        fn base(&mut self) -> &mut renderer::html::Html {
            &mut self.html
        }

        // fn code_block(&mut self, output: &mut Buffer, code: &Buffer, lang: &Buffer) {
        //     use std::io::Write;

        //     let lang = if lang.is_empty() {
        //         "text"
        //     } else {
        //         lang.to_str().unwrap()
        //     };

        //     write!(output,
// r#"<figure class="codeblock">
// <pre>
// <code class="highlight language-{}">"#, lang).unwrap();

        //     output.pipe(code);

        //     output.write(b"</code></pre></figure>").unwrap();
        // }

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


