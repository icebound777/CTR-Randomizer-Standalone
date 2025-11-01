pub struct SeedSettings {
    pub randomization: RandomizationSettings,
    pub general: GeneralSettings,
    pub qol: QualityOfLifeSettings,
}

pub struct RandomizationSettings {
    pub shuffle_adventure: bool,
    pub shuffle_race_rewards: bool,
    pub shuffle_warppads: bool,
    pub warppad_unlock_requirements: WarppadUnlockRequirements,
    pub autounlock_ctrchallenge_relicrace: bool,
}

pub struct GeneralSettings {
    pub rr_required_minimum_time: RelicTime,
    pub rr_require_perfects: bool,
    pub oxide_final_challenge_unlock: FinalOxideUnlock,
}

pub struct QualityOfLifeSettings {
    pub skip_mask_hints: bool,
    pub autoskip_podium_cutscenes: bool,
    pub skip_mask_congrats: bool,
}

pub enum WarppadUnlockRequirements {
    Vanilla = 0,
    Shuffled = 1,
    MoreChoices = 2,
    Wild = 3,
    Chaotic = 4,
}

pub enum FinalOxideUnlock {
    SappireRelics18 = 0,
    GoldAndPlatinumRelics18 = 1,
}

pub enum RelicTime {
    SapphireTime = 0,
    GoldTime = 1,
    PlatinumTime = 2,
}

impl TryFrom<i32> for WarppadUnlockRequirements {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == WarppadUnlockRequirements::Vanilla as i32 => {
                Ok(WarppadUnlockRequirements::Vanilla)
            }
            x if x == WarppadUnlockRequirements::Shuffled as i32 => {
                Ok(WarppadUnlockRequirements::Shuffled)
            }
            x if x == WarppadUnlockRequirements::MoreChoices as i32 => {
                Ok(WarppadUnlockRequirements::MoreChoices)
            }
            x if x == WarppadUnlockRequirements::Wild as i32 => Ok(WarppadUnlockRequirements::Wild),
            x if x == WarppadUnlockRequirements::Chaotic as i32 => {
                Ok(WarppadUnlockRequirements::Chaotic)
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for RelicTime {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == RelicTime::SapphireTime as i32 => Ok(RelicTime::SapphireTime),
            x if x == RelicTime::GoldTime as i32 => Ok(RelicTime::GoldTime),
            x if x == RelicTime::PlatinumTime as i32 => Ok(RelicTime::PlatinumTime),
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for FinalOxideUnlock {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == FinalOxideUnlock::SappireRelics18 as i32 => {
                Ok(FinalOxideUnlock::SappireRelics18)
            }
            x if x == FinalOxideUnlock::GoldAndPlatinumRelics18 as i32 => {
                Ok(FinalOxideUnlock::GoldAndPlatinumRelics18)
            }
            _ => Err(()),
        }
    }
}
