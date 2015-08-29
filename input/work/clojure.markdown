---
title = "Clojure & ClojureScript"
published = "August 27, 2015"
comments = false
---

## Levee: Web Interface for rtorrent

[Levee] is a web interface for [rtorrent]. The back-end is written in Clojure using a combination of [http-kit] and [compojure], while the front-end is written in ClojureScript using [Om], which itself is built on top of Facebook's [React].

[Levee]: https://github.com/blaenk/levee
[rtorrent]: http://rakshasa.github.io/rtorrent/
[http-kit]: http://http-kit.org
[compojure]: https://github.com/weavejester/compojure
[Om]: https://github.com/swannodette/om
[React]: http://facebook.github.io/react/

It consists of a clean, responsive UI with support for drag-and-drop file uploads, WebSockets for up-to-date information, and a simple locking system to facilitate a multi-user environment.

Torrent metadata pertaining to Levee is stored in the torrent itself, thereby avoiding the need to maintain consistency between rtorrent and a separate database.
