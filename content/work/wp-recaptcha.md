+++
title = "WP-reCAPTCHA"
date = 2015-08-27

[work]
kind = "project"
+++

Back in 2007 I found out about [reCAPTCHA](http://www.google.com/recaptcha), a project started at [Carnegie Mellon University](http://en.wikipedia.org/wiki/Carnegie_Mellon_University) in part by the original creator of the [CAPTCHA](http://en.wikipedia.org/wiki/CAPTCHA). I'd bet by now most people have used and are familiar with reCAPTCHA. CAPTCHAs generally consist of random letters and numbers to test if someone was a human or an automated program (usually with intent to spam). reCAPTCHA's innovation was to use actual words, words that had failed automatic digitizing when scanning mass amounts of books and using [Optical Character Recognition](http://en.wikipedia.org/wiki/Optical_character_recognition) software.

The consequence of this was that programs couldn't be written, using computer vision techniques, to simply read the image of the word, since industry-grade Optical Character Recognition software---presumably employing the most bleeding edge and sophisticated computer vision techniques---had already failed to digitize the words. This way, two problems were solved with reCAPTCHA: it was foolproof against automated attacks and circumvention, and users came to realize that by solving reCAPTCHA challenges they were helping to digitize books and thus was not a waste of time as solving randomly generated CAPTCHAs was.

I also found out about a service they had called [MailHide](http://en.wikipedia.org/wiki/ReCAPTCHA#Derivative_projects) in which they would hide part of an email address away from spammers. The full email address could be retrieved by solving a reCAPTCHA challenge, effectively preventing automated programs from harvesting email addresses for the purposes of spamming them.

That same year (2007) I figured it would be neat to write a plugin for [WordPress](http://wordpress.org/) which automatically hid any email address in a WordPress site using the MailHide technique. It gained some attention, and the then lead engineer of reCAPTCHA, Ben Maurer, contacted me via email asking if I'd be interested in volunteering to work on the then official reCAPTCHA WordPress plugin, with MailHide integrated. I accepted and worked on the plugin for a few years over which it has gotten [over 400,000 downloads](http://wordpress.org/extend/plugins/wp-recaptcha/stats/).

reCAPTCHA was acquired by Google in 2009, and the original team seems to have largely left the project. I stopped using WordPress and as a result development of the plugin halted. The plugin is open source, however, and available [on github](http://github.com/blaenk/wp-recaptcha).
