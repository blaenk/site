+++
title = "Instagib Project"
date = 2015-08-27

[work]
kind = "project"
+++

For the longest time, my favorite competitive game was a particular kind of instagib mod for the [Jedi Outcast](http://en.wikipedia.org/wiki/Star_Wars_Jedi_Knight_II:_Jedi_Outcast) (JO) and [Jedi Academy](http://en.wikipedia.org/wiki/Star_Wars_Jedi_Knight:_Jedi_Academy) (JA, the sequel) games called [disruption instagib](http://archives.thejediacademy.net/index.php/Disruption). I mainly played this on Jedi Outcast which was the older one of the two, because the server I preferred to play on in Jedi Academy shut down but one still existed in Jedi Outcast (somehow). Given that this was a pretty niche mod in a pretty old game (considering Jedi Academy had already been released), I wished to somehow make it available to more people.

Both of these games used the Quake 3 engine ([id Tech 3](http://en.wikipedia.org/wiki/Id_Tech_3)), so when the Quake 3 source code was released under the GPL and the source was cleaned up and optimized by the [ioquake3](http://ioquake3.org) project, I decided to try to port the mod and the feel of JO/JA into a standalone mod. The reason for wanting to make it into a standalone game was because although instagib mods have been around for a very long time for pretty much any game, they tend to be relegated to just that: mods. As a result, you have a variety of different flavors of instagib, who's play-style is determined by the game for which it is a mod. This is fine, but it has the effect of fragmenting the instagib community. As a result, there are usually few servers available.

So in 2006-2007 I decided to develop a standalone Instagib game. The game used art assets from the OpenArena project with custom UI and other assets designed by two of my friends. The game had team-colored rail shots and rail jumping was implemented, aside from traditional instagib mechanics. I had written a custom [NSIS](http://nsis.sourceforge.net/Main_Page) installer script to generate an installer binary for Windows. I also had Linux tarballs and Mac OS X application bundles. Aside from this, I had developed a build and deployment system with Python, which allowed people to have the latest versions of binaries and art assets.

 I ultimately abandoned the project as I became distracted by other projects. The source used to be on a self-hosted subversion server back when it was actively developed. I intend to push the source to github in the near future.

Recently, however---as a result of Disney [shutting down](http://en.wikipedia.org/wiki/LucasArts#Acquisition_by_Disney_and_closure_of_the_development_arm) LucasArts---[Raven Software](http://en.wikipedia.org/wiki/Raven_Software), the creators of Jedi Outcast and Jedi Academy, decided to release the source code to both games under the GPL. I look forward to developing a canonical disruption instagib mod again.
