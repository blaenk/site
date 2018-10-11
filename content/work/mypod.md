+++
title = "MyPod"
date = 2015-08-27

[work]
kind = "project"
+++

MyPod was an iPod music browser and retriever.

This was my first C# application which I wrote back in 2006-2007. I created it in response to an instance in which I wanted to back-up some of the music I had transferred to my iPod. The application understood the file structure of the iPod as it was back then, which consisted of several nested directories each storing about four audio files with seemingly randomly generated names. MyPod simply walked this file structure and used [TagLib](http://taglib.github.io/) to expose the actual file information to the user in a data list. The user then specified which files they wanted to back up and they were then transferred to a location and naming template (i.e. artist - title) of their choosing.
