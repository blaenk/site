+++
title = "Syncplay"
date = 2015-08-27

[work]
kind = "project"
+++

One of my most used programs is a little program called [syncplay] which works with different media players---I use [MPC-HC] on Windows and [mpv] on Linux---and makes it easy to watch movies and shows with others, automatically synchronized, so that regardless of whether someone pauses, seeks, etc., everyone else's position will be synchronized. This allows me to watch movies and shows regularly with distant friends and family.

I contributed features to the synchronization algorithm and made Syncplay packageable on Linux, then created ArchLinux AUR packages for them.

### Allow disabling rewind-on-desync

In particular, one time when about 5-6 of us were watching a movie, I was on a particularly bad connection at the time and the synchronization algorithm was being overly aggressive with compensating for this by constantly rewinding others back a few seconds. This was obviously very jarring. As long as we all started the player at more or less the same time, our machines were able to produce a steady frame rate so the players shouldn't desynchronize by any meaningful amount [^sync], so we didn't care too much about synchronization during playback due to slow clients.

After enduring a few minutes of this jarring interruption, I paused the movie and asked my friends to give me a few minutes. I quickly downloaded the Syncplay client for the first time and narrowed down the area responsible for this, rebuilt the client, and shared it with my friends, allowing us to watch the rest of the movie without any further interruptions.

Later on I polished it up and [filed a PR](https://github.com/Syncplay/syncplay/pull/24) exposing extra options in the client.

[^sync]: Or maybe I'm naive in thinking that. I'm well aware that synchronization even between audio and video of the same file can be tricky to nail down

### ArchLinux Packages

There exist user friendly installers for Windows, but Linux' side of things consist of a typical Makefile, so I created archlinux packages for the release and [git] versions of syncplay. This required some [modifications] to syncplay which were merged upstream. Furthermore, archlinux uses Python 3 by default, which syncplay can't use because of its dependency on twisted, which as far as I know is still not Python 3 compatible. So the package required some automated replacing of the shebang from `python` to `python2`.

See the PRs [here](https://github.com/Syncplay/syncplay/pull/30) and [here](https://github.com/Syncplay/syncplay/pull/37).

[git]: https://aur.archlinux.org/packages/syncplay-git/
[mpc-hc]: http://mpc-hc.org
[mpv]: http://mpv.io
[syncplay]: http://syncplay.pl
[git package]: https://aur.archlinux.org/packages/syncplay-git/
[modifications]: https://github.com/Uriziel/syncplay/pull/30
