---
title = "Git"
published = "May 4, 2016"
excerpt = "Distributed Version Control System"
comments = false
---

When merging a feature branch it's generally preferable to preserve the fact that the commits come from the feature branch for historical purposes. However, if the destination branch (e.g. master) hasn't diverged, a merge by default will be fast-forward, essentially resetting the master to the feature branch, thereby erasing any trace of a merge ever taking place, let alone from which branch.

To force git to perform a non-fast-forwarding merge, use the `--no-ff` option:

``` console
$ git merge --no-ff feature-branch
```

To ensure a fast-forwarding merge, the destination branch must not have diverged from the base of the source branch. If it did, the destination branch must be rebased on top of the destination branch.

``` console
$ git rebase new-base source
```

The history range available for an interactive rebase is the range beginning with the specified commit. For example, to rebase local commits on top of a remote branch that may have diverged, it suffices to specify the refspec of the remote branch.

``` console
$ git rebase -i origin/experiment
```

An interactive rebase shows commits in the order that they will be applied one by one, i.e. in chronological order from top-to-bottom, starting with the first commit at the top.

Squashing a commit during an interactive rebase causes it to be "squashed into" the commit that precedes if chronologically (i.e. the one above it), combining both commits' messages.

Marking a commit as a fixup during an interactive rebase is like squashing a commit except the commit's message is discarded, essentially adopting the message of the commit being squashed into.

Editing a commit during an interactive rebase has the effect of applying the changes of the commit on the working tree, allowing one to create separate commits from the changes as needed (via `add -p`) before continuing the rebase operation. This is useful for splitting a larger commit into smaller ones. It's even possible to completely change the commit.

Rewording a commit during an interactive rebase has the effect of picking it for the rebase, but having the rebase process pause to give one a chance to edit the commit message.

Commits can be deleted or even reordered during an interactive rebase by deleting or rearranging the lines.

The `@` symbol is an alias for `HEAD`.
