+++
title = "Node-Xmlrpc"
date = 2015-08-27

[work]
kind = "contribution"
+++

I contributed a [series of features](https://github.com/baalexander/node-xmlrpc/pulls/blaenk?direction=desc&page=1&sort=created&state=closed) to [node-xmlrpc](https://github.com/baalexander/node-xmlrpc) because I needed them in one of my projects at the time. node-xmlrpc is a package for [node.js](http://nodejs.org/) which provides an interface for [XML-RPC](http://en.wikipedia.org/wiki/XML-RPC) communication.

I added `i8` datatype support (8-byte integers, i.e. 64-bit integers) because the application I was interfacing with always used that datatype when responding with integers, regardless of whether or not it was necessary. I added `buffer` datatype support (Base64 encoded data) just a volunteer contribution, though I had no need for it.

The original stream XML-RPC parser was unable to handle chunked responses correctly for element inner-text. This meant that if a chunk ended inside an element, that inner-text would become truncated. I fixed it by continuing to collect the inner-text until an end-of-element event was fired by the XML parser.

Finally, I added support for basic HTTP authentication.
