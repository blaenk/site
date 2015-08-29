---
title = "C++"
published = "August 27, 2015"
comments = false
---

<toc/>

# Projects

## The Instagib Project

For the longest time, my favorite competitive game was a particular kind of instagib mod for the [Jedi Outcast](http://en.wikipedia.org/wiki/Star_Wars_Jedi_Knight_II:_Jedi_Outcast) (JO) and [Jedi Academy](http://en.wikipedia.org/wiki/Star_Wars_Jedi_Knight:_Jedi_Academy) (JA, the sequel) games called [disruption instagib](http://archives.thejediacademy.net/index.php/Disruption). I mainly played this on Jedi Outcast which was the older one of the two, because the server I preferred to play on in Jedi Academy shut down but one still existed in Jedi Outcast (somehow). Given that this was a pretty niche mod in a pretty old game (considering Jedi Academy had already been released), I wished to somehow make it available to more people.

Both of these games used the Quake 3 engine ([id Tech 3](http://en.wikipedia.org/wiki/Id_Tech_3)), so when the Quake 3 source code was released under the GPL and the source was cleaned up and optimized by the [ioquake3](http://ioquake3.org) project, I decided to try to port the mod and the feel of JO/JA into a standalone mod. The reason for wanting to make it into a standalone game was because although instagib mods have been around for a very long time for pretty much any game, they tend to be relegated to just that: mods. As a result, you have a variety of different flavors of instagib, who's play-style is determined by the game for which it is a mod. This is fine, but it has the effect of fragmenting the instagib community. As a result, there are usually few servers available.

So in 2006-2007 I decided to develop a standalone Instagib game. The game used art assets from the OpenArena project with custom UI and other assets designed by two of my friends. The game had team-colored rail shots and rail jumping was implemented, aside from traditional instagib mechanics. I had written a custom [NSIS](http://nsis.sourceforge.net/Main_Page) installer script to generate an installer binary for Windows. I also had Linux tarballs and Mac OS X application bundles. Aside from this, I had developed a build and deployment system with Python, which allowed people to have the latest versions of binaries and art assets.

 I ultimately abandoned the project as I became distracted by other projects. The source used to be on a self-hosted subversion server back when it was actively developed. I intend to push the source to github in the near future.

Recently, however---as a result of Disney [shutting down](http://en.wikipedia.org/wiki/LucasArts#Acquisition_by_Disney_and_closure_of_the_development_arm) LucasArts---[Raven Software](http://en.wikipedia.org/wiki/Raven_Software), the creators of Jedi Outcast and Jedi Academy, decided to release the source code to both games under the GPL. I look forward to developing a canonical disruption instagib mod again.

# Contributions

## libtorrent

### Fix I/O Multiplexing Error on Solaris

[libtorrent](https://github.com/rakshasa/libtorrent) is the Bit Torrent library used in rtorrent, developed by the same person. After fixing the [signal disposition establishment bug](#rtorrent) in hopes of fixing [rtorrent issue #51](https://github.com/rakshasa/rtorrent/issues/51) which rendered rtorrent unusable on Solaris derivatives, three months later someone confirmed that it had fixed one of their problems. However, rtorrent was now crashing with the message "Listener port received error event." I tracked this message down to libtorrent. The problem was that on Solaris, libtorrent employs `select()` for I/O multiplexing instead of the platform-specific API such as `/dev/poll` and event ports, and the signature of `select()` was seemingly misinterpreted---which is a common occurrence given documentation discrepancies.

For example, in the Linux man pages, `select()` is [prototyped as](http://man7.org/linux/man-pages/man2/select.2.html):

``` cpp
int select(int nfds, fd_set *readfds, fd_set *writefds,
           fd_set *exceptfds, struct timeval *timeout);
```

However, in the Solaris man pages it's [prototyped as](docs.oracle.com/cd/E26502_01/html/E29034/select-3c.html):

``` cpp
int select(int nfds, fd_set *readfds, fd_set *writefds,
           fd_set *errorfds, struct timeval *timeout);
```

The key difference is that on Linux, the fourth argument is named `exceptfds` whereas on Solaris it's named `errorfds`. This innocent-looking difference mistakenly gives the impression that file descriptors present in that set indicate that an I/O error has occurred on that file descriptor. However, this is not necessarily the case, as is outlined in [`select_tut(2)`](http://man7.org/linux/man-pages/man2/select_tut.2.html):

> This set is watched for "exceptional conditions". In practice, only one such exceptional condition is common: the availability of out-of-band (OOB) data for reading from a TCP socket. See `recv(2)`, `send(2)`, and `tcp(7)` for more details about OOB data. (One other less common case where `select(2)` indicates an exceptional condition occurs with pseudoterminals in packet mode; see `tty_ioctl(4)`.) After `select()` has returned, `exceptfds` will be cleared of all file descriptors except for those for which an exceptional condition has occurred.

Furthermore, the Solaris man page says:

> If a socket has a pending error, it is considered to have an exceptional condition pending. Otherwise, what constitutes an exceptional condition is file type-specific. For a file descriptor for use with a socket, it is protocol-specific except as noted below. For other file types, if the operation is meaningless for a particular file type, `select()` or `pselect()` indicates that the descriptor is ready for read or write operations and indicates that the descriptor has no exceptional condition pending.
>
> ...
>
> A socket is considered to have an exceptional condition pending if a receive operation with `O_NONBLOCK` clear for the open file description and with the `MSG_OOB` flag set would return out-of-band data without blocking. (It is protocol-specific whether the `MSG_OOB` flag would be used to read out-of-band data.) A socket will also be considered to have an exceptional condition pending if an out-of-band data mark is present in the receive queue.

rtorrent didn't use out-of-band data or pseudoterminals as far as I was aware, and after searching the Solaris man pages for a while I couldn't find more information on what else it could've been. Considering that this was only observable on Solaris derivatives, I decided that it must have been something platform-specific, perhaps Solaris was more relaxed on its criteria for what it considered to be an "exceptional condition."

The [fix I came up with](https://github.com/rakshasa/libtorrent/pull/40) involved invoking [`getsockopt()`](http://man7.org/linux/man-pages/man2/getsockopt.2.html) to retrieve the socket error associated with that file descriptor, and if there was indeed an error, follow through with throwing the exception, albeit with more descriptive information as to what the error was. If, on the other hand, there was no error, then simply do nothing.

## rtorrent

### Fix Unportable Signal Disposition Establishment on Solaris

The [rtorrent](http://libtorrent.rakshasa.no/) project makes use of signals for certain kinds of inter-thread communication. Certain users on [Solaris](http://en.wikipedia.org/wiki/Solaris_(operating_system)) reported that [rtorrent crashed](https://github.com/rakshasa/rtorrent/issues/51) as soon as signal `SIGUSR1` was delivered. Being an avid reader of the POSIX standards I was curious and felt that I might know what was wrong, figuring that correctly identifying and solving the problem would be a huge testament to the POSIX standards considering I had never used Solaris.

Some people in the issue figured that it must be a non-POSIX compliant implementation of `pthread_kill()` which was preventing the application from sending signals to specific threads. I didn't think this was the case, as Solaris' [manual page](http://docs.oracle.com/cd/E26502_01/html/E29034/pthread-kill-3c.html) for `pthread_kill()` claims that it's implemented as is intended. If it was indeed a non-compliant implementation, I figured someone else would've already encountered the issue and there would be some sort of note in the man page. In fact, Solaris is [fully POSIX-compliant](http://en.wikipedia.org/wiki/POSIX#Fully_POSIX-compliant) which is more than can be said of Linux, and yet Linux didn't exhibit this behavior.

Instead my suspicion was something else entirely. As soon as I recognized that clearly something signal-related was causing crashes on certain platforms in particular, I thought of the one glaring, well-known to be unportable system call: `signal()`. In fact, the Linux [man page](http://man7.org/linux/man-pages/man2/signal.2.html) for `signal()` says:

> The only portable use of `signal()` is to set a signal's disposition to `SIG_DFL` or `SIG_IGN`. The semantics when using `signal()` to establish a signal handler vary across systems (and POSIX.1 explicitly permits this variation); **do not use it for this purpose.**

The emphasis is theirs and goes to show how unpredictable the use of `signal()` could be. To complicate matters, the first line in the man page's notes section says:

> The effects of `signal()` in a multithreaded process are unspecified.

However, I believed that the problem lay in the possibility that Solaris provided different semantics for `signal()` from the semantics that Linux provided:

> In the original UNIX systems, when a handler that was established using `signal()` was invoked by the delivery of a signal, the disposition of the signal would be reset to `SIG_DFL`, and the system did not block delivery of further instances of the signal. [...] This was bad because the signal might be delivered again before the handler had a chance to reestablish itself.  Furthermore, rapid deliveries of the same signal could result in recursive invocations of the handler.

This behavior is known as [System V](http://en.wikipedia.org/wiki/UNIX_System_V) semantics. In other words, when a signal handler is established and then subsequently triggered, the signal disposition is reset to its default disposition, whatever that may be for the signal in question. If the handler isn't re-established, then a subsequent triggering of that signal will be handled based on the default disposition for that signal.

There is another behavior which is referred to as [BSD](http://en.wikipedia.org/wiki/Berkeley_Software_Distribution) semantics in which:

> the signal disposition is not reset, and further instances of the signal are blocked from being delivered while the handler is executing.  Furthermore, certain blocking system calls are automatically restarted if interrupted by a signal handler.

The situation on Linux is such that the kernel's `signal()` system call provides System V semantics. However, glibc 2 and later expose a wrapper function for `signal()` which instead delegates its work to the preferred---for portability and flexiblity reasons---system call [`sigaction()`](http://man7.org/linux/man-pages/man2/sigaction.2.html), called in such a way as to provide BSD semantics. This wrapper function is exposed if the `_BSD_SOURCE` [feature test macro](http://man7.org/linux/man-pages/man7/feature_test_macros.7.html) is defined, which it is by default.

Solaris doesn't have such a wrapper for `signal()`, instead exposing its bare, System V semantics system call with [`signal()`](http://docs.oracle.com/cd/E26502_01/html/E29034/signal-3c.html):

> ``` c
> void (*signal(int sig, void (*disp)(int)))(int);
> ```
> 
> If `signal()` is used, `disp` is the address of a signal handler, and `sig` is not `SIGILL`, `SIGTRAP`, or `SIGPWR`, the system first sets the signal's disposition to `SIG_DFL` before executing the signal handler.

This clearly states that the signal disposition is reset to its default disposition before executing the signal handler. Taking a look at the [default signal disposition table](http://docs.oracle.com/cd/E26502_01/html/E29033/signal.h-3head.html) for Solaris, we can see that `SIGUSR1`'s default disposition is to exit the application. Presumably, Solaris users were crashing upon the second delivery of `SIGUSR1` or any other signal established with `signal()` who's default disposition was to exit or abort (core dump).

My [patch](https://github.com/rakshasa/rtorrent/pull/127) simply consisted of switching out calls to `signal()` for purposes of establishing signal handlers with calls to `sigaction()`.

## MPC-HC

### Fix Web UI Seeking

I was interested in modifying [MPC-HC](http://mpc-hc.org/) to allow people to watch things in sync with each other, i.e. pause when someone pauses, seek when someone seeks, etc. I pulled the source from github and began looking for a good way to implement this functionality. I found the source for the web UI component of MPC-HC, which essentially provides an interface for which a web UI can be developed to control MPC-HC. I figured I could make use of this and began testing it when a friend noticed that the seeking in the existing web UI didn't work. After finding the relevant code in the MPC-HC source I found that it was a simple problem of failing to URL decode the seek parameter sent from the web UI. I submitted a [patch](https://github.com/mpc-hc/mpc-hc/pull/38) which was ultimately merged in and pushed out in [version 1.6.6](https://trac.mpc-hc.org/wiki/Changelog/1.6.6).

As for the original intent of implementing the functionality for synced playback, the MPC-HC developers told me about [Syncplay](http://syncplay.pl/) which I have used for months now to much success. The added benefit is that it isn't specific to any particular media player and is cross-platform.
