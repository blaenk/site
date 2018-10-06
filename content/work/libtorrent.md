+++
title = "Libtorrent"
date = 2015-08-27

[work]
kind = "contribution"
+++

## Fix I/O Multiplexing Error on Solaris

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
