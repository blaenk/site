+++
title = "Aura"
date = 2015-08-27

[work]
kind = "contribution"
+++

## Allow Number Parameter for Truncation

[Aura] is an [arch user repository] front-end written in Haskell. It allows for the seamless, automated installation of packages from the AUR. Aura has a flag `-As` which allows for searching the AUR, which can also accept the flags `--head` and `--tail` to show only the first or last 10 results.

The reason these flags exist instead of just using the `head` or `tail` programs is that the results that Aura outputs consist of two lines each: one for the package name and the other for the package description. For this reason, the Aura developers included these as a convenience. The problem was that these flags didn't accept a parameter, instead always defaulting to 10 items.

Usually the package I'm looking for is either the first result or within the first five, so the default of 10 results was too high.

I [contributed] a feature that allows these flags to accept an optional parameter specifying the number of items to return, for example, `--head=3` would show the first three results. The nice thing is that the parameter is optional, so that `--head` continues to default to 10 results, making the change backwards-compatible.

This feature made it into [version 1.2.3.3].

[Aura]: https://github.com/fosskers/aura
[arch user repository]: https://wiki.archlinux.org/index.php/Arch_User_Repository
[contributed]: https://github.com/fosskers/aura/pull/233
[version 1.2.3.3]: https://github.com/fosskers/aura/commit/e6069dd3571b1ee338da2b658847f2e08bb15b22#diff-4
