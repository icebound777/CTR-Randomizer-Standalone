# CTR Randomizer Handbook

## Adventure Difficulty

The adventure mode has dynamic difficulty.  
In regular races, bots will scale with your current number of trophies.  
Bosses and cups have their own scaling.

For reference, the arcade cup difficulties have the following difficulty values:

* Easy -> 160
* Medium -> 240
* Hard -> 320
* Super Hard (cheat) -> 400

| Number of trophies | Bot difficulty (regular) | Bot difficulty (hardmode cheat) |
| ----------- | ----------- | ----------- |
| 0 | 68 | 112 |
| 1 | 77 | 124 |
| 2 | 86 | 136 |
| 3 | 95 | 148 |
| 4 | 103 | 160 |
| 5 | 112 | 172 |
| 6 | 121 | 184 |
| 7 | 130 | 196 |
| 8 | 138 | 208 |
| 9 | 147 | 220 |
| 10 | 156 | 232 |
| 11 | 165 | 244 |
| 12 | 173 | 256 |
| 13 | 182 | 268 |
| 14 | 191 | 280 |
| 15 | 200 | 292 |
| 16 | 208 | 304 |

**Pity system**:
Every time you lose a race, a pity counter increases. CTR counts "losing the race" as finishing the race not in 1st, or restarting the race during the 3rd lap.  
Each track has its own pity counter.  
This pity counter starts at 0, counts up to 10, and resets when you win the race.  
Each pity level reduces the difficulty value.  
This also affects the items the player can get during a boss race (see further below).

| Pity counter / losses | Difficulty value change |
| ----------- | ----------- |
| 0 | - |
| 1 | -3 |
| 2 | -6 |
| 3 | -8 |
| 4 | -19 |
| 5 | -29 |
| 6 | -41 |
| 7 | -56 |
| 8 | -74 |
| 9 | -96 |
| 10 | -121 |

The minimum difficulty value is zero.

## Items

### Item Roulette

The item roulette quickly swapping between items is purely for show.  
The item you get is actually determined the moment the roulette stops by itself, or when the player manually stops the roulette by pressing Circle.  
There is no difference in the item chances between manually stopping the roulette and having the roulette stop on its own.  
However, since the item is determined the moment the roulette stops, one can break an item box to start the roulette, then get overtaken by another driver, and then stop the roulette to get a better item by having a worse position in the race.

### Dynamic item system

Races have a dynamic item system, changing the available items based on a few criteria, like current position in the race, current lap, and race type.

Available items in adventure mode:

* Tracking Missile x1
* Tracking Missile x3
* Bowling Bomb x1
* Bowling Bomb x3
* Power Shield
* Explosive Crate
* N. Brio's Beaker
* Aku Aku / Uka Uka Mask
* Turbo
* N. Tropy Clock
* Warp Orb

The following items are only available in battle mode:

* Super Engine
* Invisibility

#### Item chances

1st place:

* 40% - N. Brio's Beaker
* 40% - Explosive Crate
* 10% - Bowling Bomb x1
* 5% - Turbo
* 5% - Power Shield

2nd, 3rd, 4th place:

* ~19.23% - N. Brio's Beaker
* ~15.38% - Explosive Crate
* ~13.46% - Aku Aku / Uka Uka Mask
* ~9.61% - Bowling Bomb x1
* ~9.61% - Turbo
* ~9.61% - Warp Orb
* ~5.77% - Bowling Bomb x3
* ~5.77% - Power Shield
* ~5.77% - Tracking Missile x1
* ~3.85% - Tracking Missile x3
* ~1.92% - N. Tropy Clock

5th, 6th place (+ 7th, 8th place during lap 1):

* 25% - Aku Aku / Uka Uka Mask
* 15% - Warp Orb
* 10% - Turbo
* 10% - Bowling Bomb x3
* 10% - Tracking Missile x1
* 5% - N. Brio's Beaker
* 5% - Explosive Crate
* 5% - Bowling Bomb x1
* 5% - Power Shield
* 5% - Tracking Missile x3
* 5% - N. Tropy Clock

7th, 8th place (during lap 2+):

* 40% - Aku Aku / Uka Uka Mask
* 25% - Warp Orb
* 10% - Turbo
* 10% - N. Tropy Clock
* 5% - Power Shield
* 5% - Tracking Missile x1
* 5% - Tracking Missile x3

During a 5 person race (purple gem cup), being in 5th automatically gives the 7th/8th place item set.

During a boss race, being in 1st gives the regular 1st place item set.  
But being 2nd during a boss race instead gives these special item sets:

If lost to the boss fewer than 3 times:

* 55% - Tracking Missile x3
* 15% - Bowling Bomb x1
* 10% - Bowling Bomb x3
* 10% - Turbo
* 5% - N. Brio's Beaker
* 5% - Explosive Crate
* 5% - Power Shield
* 5% - Tracking Missile x1

If lost to the boss 3 times (adds Warp Orb):

* 40% - Tracking Missile x3
* 15% - Bowling Bomb x1
* 15% - Warp Orb
* 10% - Bowling Bomb x3
* 10% - Turbo
* 5% - N. Brio's Beaker
* 5% - Explosive Crate
* 5% - Power Shield
* 5% - Tracking Missile x1

If lost to the boss 4 times (adds Masks):

* 25% - Aku Aku / Uka Uka Mask
* 15% - Bowling Bomb x1
* 15% - Warp Orb
* 15% - Tracking Missile x3
* 10% - Bowling Bomb x3
* 10% - Turbo
* 5% - N. Brio's Beaker
* 5% - Explosive Crate
* 5% - Power Shield
* 5% - Tracking Missile x1

If lost to the boss 5 or more times (adds N. Tropy Clock):

* 25% - Aku Aku / Uka Uka Mask
* 15% - Bowling Bomb x1
* 15% - Warp Orb
* 10% - Bowling Bomb x3
* 10% - Turbo
* 10% - N. Tropy Clock
* 5% - N. Brio's Beaker
* 5% - Explosive Crate
* 5% - Power Shield
* 5% - Tracking Missile x1
* 5% - Tracking Missile x3

Furthermore, you can never get 3x Tracking Missiles while racing Komodo Joe, and will always be given a single Tracking Missile instead.

Finally, if a Warp Orb is currently on the map, or any racer currently holds a Warp Orb, no more Warp Orbs can be acquired from item boxes. In this case, if the game rolls a Warp Orb to give to the player, it will instead always swap it for a Tracking Missile x3 item.

## Wumpa Fruit

The player can collect up to 10 Wumpa Fruit by collecting single Wumpas on the track or by smashing Wumpa crates, which randomly give 5 to 8 Wumpa each.  
Wumpas 1 through 9 each increase the kart's speed.  
The 10th Wumpa does not increase the speed further, but "juices up" the currently held item.
