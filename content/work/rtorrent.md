+++
title = "Rtorrent"
date = 2015-08-27

[work]
kind = "contribution"
+++

## Fix Unportable Signal Disposition Establishment on Solaris

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

