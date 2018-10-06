+++
title = "Hakyll"
date = 2015-08-27

[work]
kind = "contribution"
+++

## Sub-Second Granularity

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

## Update to Work With Pandoc 1.12

A [patch](https://github.com/jaspervdj/hakyll/pull/183) that updates Hakyll to relfect the fact that Pandoc 1.12 decoupled the citations features. The citation features were provided by the [citeproc-hs](http://hackage.haskell.org/package/citeproc-hs) whose developer had been missing for some time now. The citeproc-hs package was embedded into the [pandoc-citeproc](http://hackage.haskell.org/package/pandoc-citeproc) package which contains the Pandoc-to-citeproc-hs interface. I simply modified Hakyll to use the new pandoc-citeproc package instead of citeproc-hs, as well as conform to the new citations API.

This change made it into [Hakyll 4.4.0.0](http://jaspervdj.be/hakyll/releases.html#hakyll-4.4.0.0).

## Add Default Port Option

Another [patch](https://github.com/jaspervdj/hakyll/pull/178) I created for [hakyll](http://jaspervdj.be/hakyll/), which was readily merged, adds a new field to Hakyll's `Configuration` [structure](http://hackage.haskell.org/packages/archive/hakyll/latest/doc/html/Hakyll-Core-Configuration.html) that allows one to specify the default port to use when running the preview server in Hakyll.

Before this patch, the default port was set to 8000---a port on which I already had a service listening on my system, and clients expected it there. It of course was possible to define a separate port as a command line argument, but this was necessary on every invocation of the preview server: `./site preview -p 4000`

With this patch users of Hakyll could override the port field in the `Configuration` structure so that an invocation of `./site preview` automatically listens on that defined port. To avoid breaking existing configurations, the default configuration still sets the default port to 8000, the only difference now is that it can be changed.

This change made it into [Hakyll 4.4.0.0](http://jaspervdj.be/hakyll/releases.html#hakyll-4.4.0.0).

## Fix Preview Functionality on Windows

[Hakyll](http://jaspervdj.be/hakyll/) is a static site generator, like [Jekyll](http://jekyllrb.com/), written in Haskell. At one point I decided to clean up my site source's directory structure by creating a separate provider directory. This triggered a bug in the latest stable release at the time (4.2.2.0) which caused the preview component to enter an infinite loop. The preview component simply watches files and recompiles them when you edit them for quick previewing on a locally-hosted web server. I found that this problem was indirectly solved in the unreleased master branch as a result of a significant change to the preview component that used specific operating systems' file notification APIs. That is, instead of the previous manual polling of the files, it would use [inotify](http://en.wikipedia.org/wiki/Inotify) on Linux for example.

All worked perfectly well on Linux, however, when I tried it on Windows I experienced very odd behavior in which the program seemed to freeze right after generating/compiling the site. Sometimes it would manage to output a single "L". I remembered that previously, it displayed a message such as "Listening on http://0.0.0.0:8000," so I concluded that somehow it was being interrupted. I found more evidence to back this hypothesis when I noticed that saving a file---thereby generating a file system event and triggering the callback established in the preview component of Hakyll (which simply recompiled the file that had been modified)---would cause the program to print a bit more of the message. "L" became "Lis" became "Listeni" and so on. Furthermore, attempting to load a page would time out unless a file was excessively saved to trigger the file system event callback---presumably affording the server's green thread enough time slices for it to respond to the request before the time out.

Upon analyzing the Hakyll source and noticing that it indirectly used the [foreign function interface](http://www.haskell.org/haskellwiki/FFI_Introduction) for interfacing with the host OS' file system events API, I found this relevant bit of information in the [GHC documentation](http://www.haskell.org/ghc/docs/latest/html/libraries/base/Control-Concurrent.html#g:5):

> Different Haskell implementations have different characteristics with regard to which operations block all threads.
> 
> Using GHC without the `-threaded` option, all foreign calls will block all other Haskell threads in the system, although I/O operations will not. With the `-threaded` option, only foreign calls with the unsafe attribute will block all other threads.

Compiling with the `-threaded` flag solved that problem. However, now the problem was that saving a file would yield a "permission denied" error in the Hakyll program. I eventually [came to realize](https://github.com/mdittmer/win32-notify/issues/3#issuecomment-18260415) that this was inherent behavior in the file system events API abstracted by the file system events Haskell package. The problem consisted of there being the possibility that the notification for a file having been modified, for example, would be sent and received/processed before the program (that caused that event to fire) had a chance to finish the actual writing that triggered the event to begin with. The workaround I [came up with](https://github.com/jaspervdj/hakyll/pull/155) consisted of simply attempting to open the file---success of which would indicate that the other process had already finished writing to the file---and if this was not possible, sleep for a bit before trying again.

The only other alternative we could think of was switching to a polling system for Windows. This was unfeasible because the file system events package didn't expose a way to force this, which would require us to implement it ourselves and would add significant overhead in performance, since every file would be polled periodically for changes, as opposed to this workaround which would poll a single file only if it wasn't able to open it on the first try.

This change made it into [Hakyll 4.3.0.0](http://jaspervdj.be/hakyll/releases.html#hakyll-4.3.0.0).
