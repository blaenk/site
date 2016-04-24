---
title = "Linux"
published = "April 22, 2016"
excerpt = "The Linux Programming Interface and POSIX APIs"
comments = false
---

What follows are notes on the Linux and POSIX APIs.

<toc/>

# Files

Each process has its own _file descriptor table_. Each entry in that table contains:

* the set of flags controlling the operation of the file descriptor (e.g. close-on-exec)
* a reference to the open file description.

The kernel maintains a system-wide _open file table_ of all _open file descriptions_, also known as _open file handles_. Each entry contains

* the current file offset
* the status flags, i.e. flags passed to `open()`
* file access mode (read-only, write-only, or read-write)
* signal-driven I/O settings
* reference to the i-node object for the file

Then each file system has a table of i-nodes for all files on the file system, where each entry contains:

* file type (e.g. regular, socket, FIFO) and permissions
* pointer to list of locks held on the file
* file properties (e.g. size, timestamp)

There can be multiple file descriptors associated with a single file in a single process either by duplicating file descriptors with `dup()` or `dup2()`, or inheriting them after a `fork()`. This causes them to point to the same _open file description_, which means that they both use and update the same file offset and status flags.

There can also be multiple file descriptors associated with the same file that do _not_ share the same _open file description_ by making separate calls to `open()`.

## Scatter-Gather I/O

The `pread()` and `pwrite()` functions can be used to read or write at an explicit offset, without consulting or updating the actual file offset.

The [`readv()`][readv] and [`writev()`][readv] functions can be used to perform scatter-gather I/O. They each take a structure that specifies a list of positions to write to or read from respectively, and how much. For example, the [`readv()`][readv] function reads a contiguous sequence of bytes from a file into the buffers specified. There are also `preadv()` and `pwritev()` variants which take an explicit offset.

[readv]: http://man7.org/linux/man-pages/man2/readv.2.html

## File Atomicity

The system can provide certain atomicity guarantees. The `O_EXCL` flag for `open()` ensures that the caller is the creator of the file. The `O_APPEND` flag ensures that multiple processes appending data don't overwrite each other's output [^O_APPEND].

[^O_APPEND]: Perhaps by making the file offsets a single atomic offset? Check.

## File Buffering

File buffering occurs at the kernel level as well as the standard library level of many languages, including C and C++'s.

The `fsync()` call causes the buffered data _and_ all metadata associated with the file descriptor to be flushed to the disk, forcing a _synchronized I/O file integrity completion state_. The `fdatasync()` call on the other hand causes only the buffered data to be flushed to the disk, forcing a _synchronized I/O data integrity completion state_. Using `fdatasync()` can be much faster if the metadata isn't as important, because the metadata and the data are usually stored on different parts of the disk.

The `sync()` call causes _all_ kernel buffers containing updated file information (both data and metadata) to be flushed to disk. On Linux, this call returns until the data has been transferred to the disk or its cache.

The `posix_fadvise()` call can be used to advise the kernel about the likely file access patterns. For example, if it is advised about random access, then Linux will disable file read-ahead.

# Processes

Each process has a system-wide process ID known as a PID. A process can obtain its PID via `getpid()`. Each process has a parent process, whose ID can be obtained via `getppid()`.

# Process Credentials

Each process may have many sets of user and group identifiers (UIDs and GIDs):

* real user ID and group ID
* effective user ID and group ID
* saved set-user-ID and saved-set-group ID
* file-system user ID and group ID (Linux-specific)
* supplementary group IDs

The _real user ID_ and _real group ID_ identify the user and group to which the process _belongs_, and these are inherited from the parent process, all the way back to the IDs read by a login shell from <span class="path">/etc/passwd</span>.

The _effective user ID_ and _effective group ID_ are the IDs consulted to determine the permissions granted to a process when it tries to perform certain operations. These are also used to determine whether a process can send a signal to another.

If a process' effective user ID is 0, which is the UID of root, then it has all of the privileges of the superuser, making it a _privileged process_.

A set-user-ID program is one which can set its effective user ID to the same value as the user ID (i.e. owner) of the executable file; same with set-group-ID. This is done by way of two special permission bits on an executable file: the set-user-ID and set-group-ID bits. These can be set via `chmod` for files a user owns or for any file if the user is privileged.

When a set-user-ID program is run, its effective user ID is set to be the same as the user ID of the program's file, and likewise in the case of set-group-ID. For example, if the file is owned by root then the program's effective user ID becomes that of root, making it a privileged process.

It's also possible to use set-user-ID to grant a program access to a particular resource, for example by creating a special-purpose user which has access to that file and setting the set-user-ID bit. This has the benefit of not giving it full, broad superuser privileges.

the _saved-set-user-ID_ and _saved-set-group-ID_ is a saved copy of the effective user ID and effective group ID after having loaded the set-UID and set-GID, if those bits were set, in which case it's a saved copy of the set-UID and set-GID. These saved copies allow certain system calls to switch between the saved-set-UID and the real UID, for example, allowing privileges to be temporarily dropped or regained, which is a good security practice.

The _file-system user ID_ and _file-system group ID_ are used on Linux instead of the effective user ID and effective group ID equivalents to determine the permissions of a process when performing file-system operations---otherwise the effective IDs are used for all other operations. However, usually the file-system IDs are identical to the effective IDs; the only differ when explicitly made to via `setfsuid()` and `setfsgid()`.

The _supplementary group IDs_ are additional groups to which a process belongs which are inherited from its parent.

# Process Time

The kernel separates CPU time used by a process into _user time_ (also known as virtual time), which is the amount of time executing in user mode, and _system time_, which is the amount of time spent executing in kernel mode, i.e. executing system calls or servicing page faults for the program.

# File Systems

Devices are represented by entries in the <span class="path">/dev</span> directory, and each device has a corresponding device driver which implements the standard operations such as `read()` and `write()`.

A _character device_ is one that handles data on a character-by-character basis, such as terminals and keyboards. A _block device_ is one that handles data a block at a time, where the block size depends on the type of device.

Hard disk drives store data on the disk on concentric circles called _tracks_, which are divided into _sectors_ each containing a series of _physical_ blocks.

The _seek time_ is the time it takes for the disk head must to move to the appropriate track. The _rotational latency_ is the time it takes for the appropriate sector to rotate under the head. The _transfer time_ is the time it takes for the blocks to be transferred.

Disks are divided into partitions, each of which may contain a file system. Each file system contains an _i-node_ (i.e. index node) table and there is one entry for each file on the file system, each entry containing metadata such as:

* file type
* user and group owners
* access permissions
* timestamps
* hard link count
* byte size
* allocated block count
* pointers to data blocks

Journaling file systems are ones keep a write-ahead-log (WAL) for metadata updates (and optionally data updates) before the actual file updates are performed, which makes the data more resilient in the event of a crash. This also means that a file system check is not required after a system crash.

File systems are all mounted under a single directory tree represented by the root <span class="path">/</span> at specific locations known as _mount points_. The _virtual file system_ (VFS) is an abstraction, a unified representation of a single file system, even if there are multiple different file systems mounted at different points.

A _bind mount_ allows a file or directory to be mounted at multiple locations in the file system. Unlike hard links, bind mounts can cross file-systems and chroot jails, and it's possible to create a bind mount for a directory.
