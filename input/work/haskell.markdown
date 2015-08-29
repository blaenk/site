---
title = "Haskell"
published = "August 27, 2015"
comments = false
---

<toc/>

# Projects

## Heavily Customized Hakyll Website

I wouldn't include my website as a project if it weren't for the fact that I have heavily modified it. I've [written] about many of these modifications and customizations. Some of the features I've implemented as Pandoc abstract syntax tree transformers and others as Hakyll custom compilers.

Pandoc AST transformers I've written include blockquote beautification, automatic abbreviation substitution, [Pygments] codeblock highlighting, and table of contents generation (with pure CSS nested section numbering).

Custom Hakyll compilers I've written include a [preview-draft system] to separate drafts from other published posts and another to [automatically push changes] as they're written to the client through WebScokets, which is useful when previewing a post that's in the middle of being written.

A full list of customizations is available in the readme for the [repository].

[written]: http://localhost:4000/tags/hakyll/
[Pygments]: http://pygments.org/
[preview-draft system]: /posts/drafts-in-hakyll/
[automatically push changes]: /posts/live-editing-with-hakyll/
[repository]: https://github.com/blaenk/blaenk.github.io

## Pulse Visualizer

This was my first Haskell application, aside from exercise solutions to Haskell books. During my final semester of college in 2012, I wanted to do some Independent Study to round out full-time student status. A [professor](http://kevinwortman.com/) agreed to mentor me in two different independent studies: [digital signal processing](http://en.wikipedia.org/wiki/Digital_signal_processing) and [Haskell](http://en.wikipedia.org/wiki/Haskell_(programming_language)). At first I had intended on treating them separately with the goal of writing a music visualizer for iTunes for the DSP study and perhaps a web application for the Haskell study. My professor suggested I try and merge them to make it easier on myself and that is exactly what I did.

I had already gotten a barebones iTunes visualizer up and running with C, so I figured I would write some hooks with the [foreign function interface](http://en.wikipedia.org/wiki/Foreign_function_interface) to delegate most of the work to Haskell. The way of going about this was pretty messy however, as it involved (at the time, and most likely even now) compiling the Haskell code into dynamic link libraries because the Haskell code had to be compiled with gcc, who's symbols differed from the ones Visual Studio produced, which I wanted to use to take advantage of DirectX 11 and DirectCompute.

I managed to get something working, but it felt very messy and was quite the abomination: Haskell to DLL with GCC on Windows linked with an iTunes Visualization Plugin DLL produced by MSVC which used DirectX 11. So I decided to instead look around for options to pursue on Linux, where Haskell development felt a lot more natural to me. After looking around for xmms, Banshee, or other bindings, and finding them lacking, I figured I might as well create a visualizer for a more fundamental thing: [PulseAudio](http://en.wikipedia.org/wiki/PulseAudio) itself.

PulseAudio has a concept of sources (e.g. processes) and sinks (e.g. sound cards). Every sink also has a corresponding source known as a monitor, meaning that the audio going to the associated sink can be intercepted and read. I found a [binding for Haskell](http://hackage.haskell.org/package/pulse-simple) that seemed sufficient enough which allowed me to monitor all of the audio on the system. I then paired this up with OpenGL to draw a pretty basic "frequency bar" visualization. The major benefit of having written it for PulseAudio itself instead of a particular music player or even as a standalone application is that I could then play the audio anywhere, such as YouTube or Pandora, and watch it visualized in my application.

Source is available [on github](https://github.com/blaenk/pulse-visualizer).

# Contributions

## Aura

### Allow Number Parameter for Truncation

[Aura] is an [arch user repository] front-end written in Haskell. It allows for the seamless, automated installation of packages from the AUR. Aura has a flag `-As` which allows for searching the AUR, which can also accept the flags `--head` and `--tail` to show only the first or last 10 results.

The reason these flags exist instead of just using the `head` or `tail` programs is that the results that Aura outputs consist of two lines each: one for the package name and the other for the package description. For this reason, the Aura developers included these as a convenience. The problem was that these flags didn't accept a parameter, instead always defaulting to 10 items.

Usually the package I'm looking for is either the first result or within the first five, so the default of 10 results was too high.

I [contributed] a feature that allows these flags to accept an optional parameter specifying the number of items to return, for example, `--head=3` would show the first three results. The nice thing is that the parameter is optional, so that `--head` continues to default to 10 results, making the change backwards-compatible.

This feature made it into [version 1.2.3.3].

[Aura]: https://github.com/fosskers/aura
[arch user repository]: https://wiki.archlinux.org/index.php/Arch_User_Repository
[contributed]: https://github.com/fosskers/aura/pull/233
[version 1.2.3.3]: https://github.com/fosskers/aura/commit/e6069dd3571b1ee338da2b658847f2e08bb15b22#diff-4

## Hakyll

### Sub-Second Granularity

[Hakyll] has a watch mode where it watches files and re-compiles them when they're modified. It does this by checking the file's modified time and storing it for comparison on the next iteration. When the new major-release version 7.8 of [GHC] was released, a [bug was observed] in Hakyll where whenever any file was modified, the whole site was recompiled instead of just the file that was modified.

[Hakyll]: http://jaspervdj.be/hakyll/
[GHC]: http://en.wikipedia.org/wiki/Glasgow_Haskell_Compiler
[bug was observed]: https://github.com/jaspervdj/hakyll/issues/250

The perplexing thing was that the Hakyll source hadn't changed at all, so clearly the bug was being caused by a direct or indirect package dependency or, _gulp_, the compiler itself. All in all, it seemed pretty uncharacteristic of my impression of Haskell.

Of course, it wasn't a compiler bug. Instead, it seemed to have been caused by the culmination of different factors. First, the [directory] package, which houses the `getModificationTime` function used to retrieve a file's modification time, began supporting sub-second precision in version 1.2 and above _if_ it's linked against the [unix] package version 2.6 and above.

> Note: When linked against unix-2.6.0.0 or later the reported time supports sub-second precision if provided by the underlying system call.
>
> <cite>[documentation][getModificationTime] for `getModificationTime`</cite>

[directory]: http://hackage.haskell.org/package/directory
[unix]: http://hackage.haskell.org/package/unix
[getModificationTime]: http://hackage.haskell.org/package/directory-1.2.1.0/docs/System-Directory.html#v:getModificationTime

The problem was essentially that Hakyll cached the modification time by first shaving off the sub-second precision with the `floor` function. However, when it then compared against this cached modification time, the other comparison operand's sub-second precision _wasn't_ shaved. What this meant was that the file was _almost always_ deemed modified, unless of course the modification time had a sub-second count of zero to begin with.

To illustrate the problem:

1. Read modification time as 3:45.325, shave sub-seconds and save as 3:45.000.
2. Read modification time as 3:45.325, compare against cached modification time, 3:45.000, to see if the file has changed.
3. 3:45.325 is more recent than 3:45.000, so the file is considered to have been modified.

The [patch][sub-second patch] simply _kept_ the sub-second precision when caching, allowing for graceful handling of systems that do and don't support sub-second precision.

This fix made it into [Hakyll 4.5.2.0].

[sub-second patch]: https://github.com/jaspervdj/hakyll/pull/252
[Hakyll 4.5.2.0]: https://github.com/jaspervdj/hakyll/commit/d89fadcdb97c2acd9aeaa58c830d30ad755f31d7

### Update to Work With Pandoc 1.12

A [patch](https://github.com/jaspervdj/hakyll/pull/183) that updates Hakyll to relfect the fact that Pandoc 1.12 decoupled the citations features. The citation features were provided by the [citeproc-hs](http://hackage.haskell.org/package/citeproc-hs) whose developer had been missing for some time now. The citeproc-hs package was embedded into the [pandoc-citeproc](http://hackage.haskell.org/package/pandoc-citeproc) package which contains the Pandoc-to-citeproc-hs interface. I simply modified Hakyll to use the new pandoc-citeproc package instead of citeproc-hs, as well as conform to the new citations API.

This change made it into [Hakyll 4.4.0.0](http://jaspervdj.be/hakyll/releases.html#hakyll-4.4.0.0).

### Add Default Port Option

Another [patch](https://github.com/jaspervdj/hakyll/pull/178) I created for [hakyll](http://jaspervdj.be/hakyll/), which was readily merged, adds a new field to Hakyll's `Configuration` [structure](http://hackage.haskell.org/packages/archive/hakyll/latest/doc/html/Hakyll-Core-Configuration.html) that allows one to specify the default port to use when running the preview server in Hakyll.

Before this patch, the default port was set to 8000---a port on which I already had a service listening on my system, and clients expected it there. It of course was possible to define a separate port as a command line argument, but this was necessary on every invocation of the preview server: `./site preview -p 4000`

With this patch users of Hakyll could override the port field in the `Configuration` structure so that an invocation of `./site preview` automatically listens on that defined port. To avoid breaking existing configurations, the default configuration still sets the default port to 8000, the only difference now is that it can be changed.

This change made it into [Hakyll 4.4.0.0](http://jaspervdj.be/hakyll/releases.html#hakyll-4.4.0.0).

### Fix Preview Functionality on Windows

[Hakyll](http://jaspervdj.be/hakyll/) is a static site generator, like [Jekyll](http://jekyllrb.com/), written in Haskell. At one point I decided to clean up my site source's directory structure by creating a separate provider directory. This triggered a bug in the latest stable release at the time (4.2.2.0) which caused the preview component to enter an infinite loop. The preview component simply watches files and recompiles them when you edit them for quick previewing on a locally-hosted web server. I found that this problem was indirectly solved in the unreleased master branch as a result of a significant change to the preview component that used specific operating systems' file notification APIs. That is, instead of the previous manual polling of the files, it would use [inotify](http://en.wikipedia.org/wiki/Inotify) on Linux for example.

All worked perfectly well on Linux, however, when I tried it on Windows I experienced very odd behavior in which the program seemed to freeze right after generating/compiling the site. Sometimes it would manage to output a single "L". I remembered that previously, it displayed a message such as "Listening on http://0.0.0.0:8000," so I concluded that somehow it was being interrupted. I found more evidence to back this hypothesis when I noticed that saving a file---thereby generating a file system event and triggering the callback established in the preview component of Hakyll (which simply recompiled the file that had been modified)---would cause the program to print a bit more of the message. "L" became "Lis" became "Listeni" and so on. Furthermore, attempting to load a page would time out unless a file was excessively saved to trigger the file system event callback---presumably affording the server's green thread enough time slices for it to respond to the request before the time out.

Upon analyzing the Hakyll source and noticing that it indirectly used the [foreign function interface](http://www.haskell.org/haskellwiki/FFI_Introduction) for interfacing with the host OS' file system events API, I found this relevant bit of information in the [GHC documentation](http://www.haskell.org/ghc/docs/latest/html/libraries/base/Control-Concurrent.html#g:5):

> Different Haskell implementations have different characteristics with regard to which operations block all threads.
> 
> Using GHC without the `-threaded` option, all foreign calls will block all other Haskell threads in the system, although I/O operations will not. With the `-threaded` option, only foreign calls with the unsafe attribute will block all other threads.

Compiling with the `-threaded` flag solved that problem. However, now the problem was that saving a file would yield a "permission denied" error in the Hakyll program. I eventually [came to realize](https://github.com/mdittmer/win32-notify/issues/3#issuecomment-18260415) that this was inherent behavior in the file system events API abstracted by the file system events Haskell package. The problem consisted of there being the possibility that the notification for a file having been modified, for example, would be sent and received/processed before the program (that caused that event to fire) had a chance to finish the actual writing that triggered the event to begin with. The workaround I [came up with](https://github.com/jaspervdj/hakyll/pull/155) consisted of simply attempting to open the file---success of which would indicate that the other process had already finished writing to the file---and if this was not possible, sleep for a bit before trying again.

The only other alternative we could think of was switching to a polling system for Windows. This was unfeasible because the file system events package didn't expose a way to force this, which would require us to implement it ourselves and would add significant overhead in performance, since every file would be polled periodically for changes, as opposed to this workaround which would poll a single file only if it wasn't able to open it on the first try.

This change made it into [Hakyll 4.3.0.0](http://jaspervdj.be/hakyll/releases.html#hakyll-4.3.0.0).

## Haxr

### i8-Type Support

[HaXR] is the main (only?) available package for performing XML-RPC in Haskell. It supports type conversions via typeclasses as with other serialization packages such as [Aeson], which can also be automated via [Template Haskell]. I [contributed][haxr pr] `i8`-type support to the package.

[HaXR]: http://hackage.haskell.org/package/haxr
[Aeson]: http://hackage.haskell.org/package/aeson
[Template Haskell]: http://www.haskell.org/haskellwiki/Template_Haskell
[haxr pr]: https://github.com/byorgey/haxr/pull/1
