+++
title = "Vim-Pandoc-Syntax"
date = 2015-08-27

[work]
kind = "contribution"
+++

## Embedded Codeblock Highlighting, Various Features & Fixes

I contributed a [variety] of features to the [vim-pandoc-syntax] plugin, a plugin that provides [Pandoc-flavored markdown] syntax highlighting and concealment to vim. Concealments in vim are a relatively new feature which allow a different in-editor appearance for certain patterns. For example, the underscores that are used to italicize text can be concealed to decrease clutter.

I started out by fixing the italic pattern, which would previously get tripped up by intra-word underscores such as in ALL_BUILD. Then I added concealment of codeblock delimiters, so that the starting delimiter for codeblocks gets replaced with a &lambda; and the end delimiter gets concealed altogether. I also added abbreviation highlighting and concealment, then I added strong-emphasis highlighting and concealment. I fixed a bug in the definition block pattern that was causing cascading issues with other patterns.

The most substantial contribution was [embedded-language highlighting] for codeblocks. That is, if one writes a Haskell codeblock, the codeblock's contents would be highlighted using the Haskell syntax highlighter:

<img src="//i.imgur.com/WpK6jNZ.png" class="center">

[vim-pandoc-syntax]: https://github.com/vim-pandoc/vim-pandoc-syntax
[variety]: https://github.com/vim-pandoc/vim-pandoc-syntax/pulls/blaenk?direction=desc&page=1&sort=created&state=closed
[Pandoc-flavored markdown]: http://johnmacfarlane.net/pandoc/README.html#pandocs-markdown
[embedded-language highlighting]: https://github.com/vim-pandoc/vim-pandoc-syntax/issues/14
