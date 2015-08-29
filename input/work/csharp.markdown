---
title = "C#"
published = "August 27, 2015"
comments = false
---

## Musicaster: Homemade last.fm

This was an application I wrote back in 2006-2007 which was a combination of client-side C# and server-side PHP. The application was basically similar to [last.fm](http://last.fm), in which a client-side application scans popular media players for the currently playing song and then transmits the information to a server-side endpoint for aggregation. The client-side C# application was written with modularity in mind, so that one would simply implement an interface, generate a DLL, and place it in the same directory as the executable to add support for more players. I added such plugins for iTunes, Windows Media Player, and Winamp. Information about the currently playing song was then sent to an endpoint of the user's choosing. At the time I also wrote a WordPress plugin which displayed this information in a blog's description text (usually under the blog title/name).

After I had done all this, one of the friends I showed it to said, "Oh, so it's like last.fm?" I had never heard of last.fm before this and the whole time I had thought I had created something quite innovative.

## MyPod: iPod Music Navigator and Retriever

This was my first C# application which I wrote back in 2006-2007. I created it in response to an instance in which I wanted to back-up some of the music I had transferred to my iPod. The application understood the file structure of the iPod as it was back then, which consisted of several nested directories each storing about four audio files with seemingly randomly generated names. MyPod simply walked this file structure and used [TagLib](http://taglib.github.io/) to expose the actual file information to the user in a data list. The user then specified which files they wanted to back up and they were then transferred to a location and naming template (i.e. artist - title) of their choosing.
