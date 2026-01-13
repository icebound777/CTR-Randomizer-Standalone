# Changelog

## alpha 3

### Features (seed generator)

* Auto-Unlock CTR Challenge & Relic Race
  * If this setting is turned on, then all warp pads' second stage unlock, the CTR Challenge and Relic Race, is always "free" and available right after beating the warp pad's Trophy Race.  
  Does not affect battle arena warp pads, gem cup warp pads, or relic-only warp pads.
* Spoiler log
  * Now includes the settings chosen for the seed.

### Bug Fixes (seed generator)

* Oxide's Final Challenge Unlock
  * Fix an issue where the Final Challenge Unlock dropdown menu did absolutely nothing, and that setting instead falsely used the "Relic Races: Required Minimum Time" dropdown. This resulted in unexpected requirements for Oxide's Final Challenge, and the seed generator crashing if "Relic Races: Required Minimum Time" was set to "Platinum Time".

## alpha 2

### Features (seed generator) (alpha 2)

* 'About' tab
  * Added a quick-link to the CTR Randomizer Handbook

### Bug Fixes (seed generator) (alpha 2)

* Gem Cup Warp Pad Shuffle
  * Reduced the chance of the seed generator crashing when shuffling gem cup warp pads from ~32% to ~0.02%. This was achieved by two changes:
    * Fixing a logic bug with those locations, which applied a "2 key" requirement to gem cup warp pads, regardless of their location in the warp pad shuffle.
    * Increasing the "seed generation failure retry" from 10 times to 1000 times.

### Bug Fixes (base mod) (alpha 2)

* Gem Cups
  * Exiting out of a gem cup, or losing a gem cup, should no longer place the player into an unloaded hub. This only happened if gem cup warp pads were shuffled into locations outside of Gemstone Valley.
* Oxide Cutscene
  * The "true ending" post-Oxide cutscene is no longer triggered by having won the Sewer Speedway CTR Challenge. Instead requires beating Oxide's Final Challenge.
* Credits
  * The "true ending" cutscene should no longer play if the player has beaten all sapphire relic times. Instead requires beating Oxide's Final Challenge.
  * The "true ending with confetti" aka "101% credits" should no longer play if the player has beaten all gold relic times. Instead actually requires 101% completion.

### CTR Randomizer Handbook (alpha 2)

* Added info about the different Oxide Cutscenes and Credits.
* Explained Track Champions.
* Explained how the adventure mode completion percentage is calculated.

## alpha 1

### Features (alpha 1)

* HUD Relic Count
  * During the post-race cutscenes, if the player wins a relic, now locks the color of the HUD relic to the color of the relic the player won for a few seconds. This makes it easier to see the new count of that specific relic.

### Bug Fixes (base mod) (alpha 1)

* Relic models
  * Fix wrong relic color during the post-race cutscenes.
  * Fix the HUD relic sometimes showing a count of "-1" duringthe post-race cutscenes.
* Relic Races
  * Fix starting the player in the 8th position at the starting line. Now instead uses the special time trial starting positions.
