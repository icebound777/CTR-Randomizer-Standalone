# Changelog

## alpha 2

### Bug Fixes (seed generator)

* Gem Cup Warp Pad Shuffle
  * Reduced the chance of the seed generator crashing when shuffling gem cup warp pads from ~32% to ~0.02%. This was achieved by two changes:
    * Fixing a logic bug with those locations, which applied a "2 key" requirement to gem cup warp pads, regardless of their location in the warp pad shuffle.
    * Increasing the "seed generation failure retry" from 10 times to 1000 times.

### Bug Fixes (base mod)

* Gem Cups
  * Exiting out of a gem cup, or losing a gem cup, should no longer place the player into an unloaded hub. This only happened if gem cup warp pads were shuffled into locations outside of Gemstone Valley.
