+++
title = "Pulse Visualizer"
date = 2015-08-27

[work]
kind = "project"
+++

## Pulse Visualizer

This was my first Haskell application, aside from exercise solutions to Haskell books. During my final semester of college in 2012, I wanted to do some Independent Study to round out full-time student status. A [professor](http://kevinwortman.com/) agreed to mentor me in two different independent studies: [digital signal processing](http://en.wikipedia.org/wiki/Digital_signal_processing) and [Haskell](http://en.wikipedia.org/wiki/Haskell_(programming_language)). At first I had intended on treating them separately with the goal of writing a music visualizer for iTunes for the DSP study and perhaps a web application for the Haskell study. My professor suggested I try and merge them to make it easier on myself and that is exactly what I did.

I had already gotten a barebones iTunes visualizer up and running with C, so I figured I would write some hooks with the [foreign function interface](http://en.wikipedia.org/wiki/Foreign_function_interface) to delegate most of the work to Haskell. The way of going about this was pretty messy however, as it involved (at the time, and most likely even now) compiling the Haskell code into dynamic link libraries because the Haskell code had to be compiled with gcc, who's symbols differed from the ones Visual Studio produced, which I wanted to use to take advantage of DirectX 11 and DirectCompute.

I managed to get something working, but it felt very messy and was quite the abomination: Haskell to DLL with GCC on Windows linked with an iTunes Visualization Plugin DLL produced by MSVC which used DirectX 11. So I decided to instead look around for options to pursue on Linux, where Haskell development felt a lot more natural to me. After looking around for xmms, Banshee, or other bindings, and finding them lacking, I figured I might as well create a visualizer for a more fundamental thing: [PulseAudio](http://en.wikipedia.org/wiki/PulseAudio) itself.

PulseAudio has a concept of sources (e.g. processes) and sinks (e.g. sound cards). Every sink also has a corresponding source known as a monitor, meaning that the audio going to the associated sink can be intercepted and read. I found a [binding for Haskell](http://hackage.haskell.org/package/pulse-simple) that seemed sufficient enough which allowed me to monitor all of the audio on the system. I then paired this up with OpenGL to draw a pretty basic "frequency bar" visualization. The major benefit of having written it for PulseAudio itself instead of a particular music player or even as a standalone application is that I could then play the audio anywhere, such as YouTube or Pandora, and watch it visualized in my application.

Source is available [on github](https://github.com/blaenk/pulse-visualizer).
