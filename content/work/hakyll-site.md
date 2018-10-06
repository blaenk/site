+++
title = "Hakyll Site"
date = 2015-08-27

[work]
kind = "project"
+++

I wouldn't include my website as a project if it weren't for the fact that I have heavily modified it. I've [written] about many of these modifications and customizations. Some of the features I've implemented as Pandoc abstract syntax tree transformers and others as Hakyll custom compilers.

Pandoc AST transformers I've written include blockquote beautification, automatic abbreviation substitution, [Pygments] codeblock highlighting, and table of contents generation (with pure CSS nested section numbering).

Custom Hakyll compilers I've written include a [preview-draft system] to separate drafts from other published posts and another to [automatically push changes] as they're written to the client through WebScokets, which is useful when previewing a post that's in the middle of being written.

A full list of customizations is available in the readme for the [repository].

[written]: http://localhost:4000/tags/hakyll/
[Pygments]: http://pygments.org/
[preview-draft system]: /posts/drafts-in-hakyll/
[automatically push changes]: /posts/live-editing-with-hakyll/
[repository]: https://github.com/blaenk/blaenk.github.io
