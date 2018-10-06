+++
title = "Syncplay for Archlinux"
date = 2015-08-27

[work]
kind = "project"
+++

One of my most used programs is a little program called [syncplay] which works with different media players---I use [MPC-HC] on Windows and [mpv] on Linux---and makes it easy to watch movies and shows with others, automatically synchronized, so that regardless of whether someone pauses, seeks, etc., everyone else's position will be synchronized. This allows me to watch movies and shows regularly with distant friends and family.

There exist user friendly installers for Windows, but Linux' side of things consist of a typical Makefile, so I created archlinux packages for the release and [git] versions of syncplay. This required some [modifications] to syncplay which were merged upstream. Furthermore, archlinux uses Python 3 by default, which syncplay can't use because of its dependency on twisted, which as far as I know is still not Python 3 compatible. So the package required some automated replacing of the shebang from `python` to `python2`.

[git]: https://aur.archlinux.org/packages/syncplay-git/
[MPC-HC]: http://mpc-hc.org
[mpv]: http://mpv.io
[syncplay]: http://syncplay.pl
[git package]: https://aur.archlinux.org/packages/syncplay-git/
[modifications]: https://github.com/Uriziel/syncplay/pull/30
