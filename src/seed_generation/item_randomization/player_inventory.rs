use crate::seed_generation::randomization_datastructures::{
    LevelID, RaceReward, RequiredItem, UnlockRequirement
};

pub struct PlayerInventory {
    num_trophies: u8,
    num_ctr_tokens_red: u8,
    num_ctr_tokens_green: u8,
    num_ctr_tokens_blue: u8,
    num_ctr_tokens_yellow: u8,
    num_ctr_tokens_purple: u8,
    num_relics_sapphire: u8,
    num_relics_gold: u8,
    num_relics_platinum: u8,
    num_keys: u8,
    collected_gems: GemFlags,
    beaten_tracks: Vec<LevelID>,
    //unlocked_tracks: TrackUnlocks,
    //unlocked_characters: CharacterUnlocks,
    //unlocked_cups: CupUnlocks,
}

impl PlayerInventory {
    pub fn new() -> Self {
        PlayerInventory {
            num_trophies: 0,
            num_ctr_tokens_red: 0,
            num_ctr_tokens_green: 0,
            num_ctr_tokens_blue: 0,
            num_ctr_tokens_yellow: 0,
            num_ctr_tokens_purple: 0,
            num_relics_sapphire: 0,
            num_relics_gold: 0,
            num_relics_platinum: 0,
            num_keys: 0,
            collected_gems: GemFlags {
                red_gem: false,
                green_gem: false,
                blue_gem: false,
                yellow_gem: false,
                purple_gem: false,
            },
            beaten_tracks: Vec::new(),
        }
    }

    pub fn add_item(&mut self, new_item: RaceReward) {
        match new_item {
            RaceReward::Trophy => {
                if self.num_trophies < 16 {
                    self.num_trophies += 1;
                }
            }
            RaceReward::RedCtrToken => {
                if self.num_ctr_tokens_red < 4 {
                    self.num_ctr_tokens_red += 1;
                }
            }
            RaceReward::GreenCtrToken => {
                if self.num_ctr_tokens_green < 4 {
                    self.num_ctr_tokens_green += 1;
                }
            }
            RaceReward::BlueCtrToken => {
                if self.num_ctr_tokens_blue < 4 {
                    self.num_ctr_tokens_blue += 1;
                }
            }
            RaceReward::YellowCtrToken => {
                if self.num_ctr_tokens_yellow < 4 {
                    self.num_ctr_tokens_yellow += 1;
                }
            }
            RaceReward::PurpleCtrToken => {
                if self.num_ctr_tokens_purple < 4 {
                    self.num_ctr_tokens_purple += 1;
                }
            }
            RaceReward::SapphireRelic => {
                if self.num_relics_sapphire < 18 {
                    self.num_relics_sapphire += 1;
                }
            }
            RaceReward::GoldRelic => {
                if self.num_relics_gold < 18 {
                    self.num_relics_gold += 1;
                }
            }
            RaceReward::PlatinumRelic => {
                if self.num_relics_platinum < 18 {
                    self.num_relics_platinum += 1;
                }
            }
            RaceReward::Key => {
                if self.num_keys < 4 {
                    self.num_keys += 1;
                }
            }
            RaceReward::RedGem
            | RaceReward::GreenGem
            | RaceReward::BlueGem
            | RaceReward::YellowGem
            | RaceReward::PurpleGem => {
                self.collected_gems.set_gem(new_item);
            }
            RaceReward::BeatTheGame => {}
        }
    }

    pub fn add_track(&mut self, track: LevelID) {
        if !self.beaten_tracks.contains(&track) {
            self.beaten_tracks.push(track);
        }
    }

    pub fn does_pass_requirements(&self, requirements: &Vec<UnlockRequirement>) -> bool {
        let mut passes_all_requirements = true;

        for req in requirements {
            match req {
                UnlockRequirement::Item(item_req) => {
                    let item_count = match item_req.item_type {
                        RequiredItem::Trophy => self.num_trophies,
                        RequiredItem::RedCtrToken => self.num_ctr_tokens_red,
                        RequiredItem::GreenCtrToken => self.num_ctr_tokens_green,
                        RequiredItem::BlueCtrToken => self.num_ctr_tokens_blue,
                        RequiredItem::YellowCtrToken => self.num_ctr_tokens_yellow,
                        RequiredItem::PurpleCtrToken => self.num_ctr_tokens_purple,
                        RequiredItem::AnyCtrToken => {
                            self.num_ctr_tokens_red
                                + self.num_ctr_tokens_green
                                + self.num_ctr_tokens_blue
                                + self.num_ctr_tokens_yellow
                                + self.num_ctr_tokens_purple
                        }
                        RequiredItem::SapphireRelic => self.num_relics_sapphire,
                        RequiredItem::GoldRelic => self.num_relics_gold,
                        RequiredItem::PlatinumRelic => self.num_relics_platinum,
                        RequiredItem::AnyRelic => {
                            self.num_relics_sapphire + self.num_relics_gold + self.num_relics_platinum
                        }
                        RequiredItem::Key => self.num_keys,
                        RequiredItem::RedGem => u8::from(self.collected_gems.red_gem),
                        RequiredItem::GreenGem => u8::from(self.collected_gems.green_gem),
                        RequiredItem::BlueGem => u8::from(self.collected_gems.blue_gem),
                        RequiredItem::YellowGem => u8::from(self.collected_gems.yellow_gem),
                        RequiredItem::PurpleGem => u8::from(self.collected_gems.purple_gem),
                        RequiredItem::AnyGem => self.collected_gems.get_count(),
                    };

                    if item_count < item_req.count {
                        passes_all_requirements = false;
                        break;
                    }
                },
                UnlockRequirement::LevelList(levels_req) => {
                    for level in levels_req {
                        if !self.beaten_tracks.contains(level) {
                            passes_all_requirements = false;
                            break;
                        }
                    }
                }
            }
        }

        passes_all_requirements
    }
}

impl Default for PlayerInventory {
    fn default() -> Self {
        PlayerInventory::new()
    }
}

struct GemFlags {
    red_gem: bool,
    green_gem: bool,
    blue_gem: bool,
    yellow_gem: bool,
    purple_gem: bool,
}

impl GemFlags {
    fn get_count(&self) -> u8 {
        u8::from(self.red_gem)
            + u8::from(self.green_gem)
            + u8::from(self.blue_gem)
            + u8::from(self.yellow_gem)
            + u8::from(self.purple_gem)
    }

    fn set_gem(&mut self, gem: RaceReward) {
        match gem {
            RaceReward::RedGem => {
                self.red_gem = true;
            }
            RaceReward::GreenGem => {
                self.green_gem = true;
            }
            RaceReward::BlueGem => {
                self.blue_gem = true;
            }
            RaceReward::YellowGem => {
                self.yellow_gem = true;
            }
            RaceReward::PurpleGem => {
                self.purple_gem = true;
            }
            _ => (),
        }
    }
}
