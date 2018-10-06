+++
title = "MPC-HC"
date = 2015-08-27

[work]
kind = "contribution"
+++

## Fix Web UI Seeking

I was interested in modifying [MPC-HC](http://mpc-hc.org/) to allow people to watch things in sync with each other, i.e. pause when someone pauses, seek when someone seeks, etc. I pulled the source from github and began looking for a good way to implement this functionality. I found the source for the web UI component of MPC-HC, which essentially provides an interface for which a web UI can be developed to control MPC-HC. I figured I could make use of this and began testing it when a friend noticed that the seeking in the existing web UI didn't work. After finding the relevant code in the MPC-HC source I found that it was a simple problem of failing to URL decode the seek parameter sent from the web UI. I submitted a [patch](https://github.com/mpc-hc/mpc-hc/pull/38) which was ultimately merged in and pushed out in [version 1.6.6](https://trac.mpc-hc.org/wiki/Changelog/1.6.6).

As for the original intent of implementing the functionality for synced playback, the MPC-HC developers told me about [Syncplay](http://syncplay.pl/) which I have used for months now to much success. The added benefit is that it isn't specific to any particular media player and is cross-platform.
