+++
title = "Hoedown"
date = 2021-11-08

[work]
kind = "contribution"
+++

Hoedown is a C library for processing and rendering Markdown as HTML. It also provides an API for hooking into the Markdown parsing.

Early on Rust lacked a fully-featured Markdown library and I needed one for the static site generator I was writing ([diecast](https://github.com/diecast/diecast)), so I wrote **idiomatic** [bindings to hoedown for Rust](https://github.com/blaenk/hoedown), and in so doing, I exposed a few previously unknown edge cases in the C library through different combinations of feature flags.

See the PRs [here](https://github.com/hoedown/hoedown/pulls?q=author%3Ablaenk+is%3Apr).
