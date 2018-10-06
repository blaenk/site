+++
title = "Musicaster"
date = 2015-08-27

[work]
kind = "project"
+++

## Musicaster: Homemade last.fm

This was an application I wrote back in 2006-2007 which was a combination of client-side C# and server-side PHP. The application was basically similar to [last.fm](http://last.fm), in which a client-side application scans popular media players for the currently playing song and then transmits the information to a server-side endpoint for aggregation. The client-side C# application was written with modularity in mind, so that one would simply implement an interface, generate a DLL, and place it in the same directory as the executable to add support for more players. I added such plugins for iTunes, Windows Media Player, and Winamp. Information about the currently playing song was then sent to an endpoint of the user's choosing. At the time I also wrote a WordPress plugin which displayed this information in a blog's description text (usually under the blog title/name).

After I had done all this, one of the friends I showed it to said, "Oh, so it's like last.fm?" I had never heard of last.fm before this and the whole time I had thought I had created something quite innovative.
