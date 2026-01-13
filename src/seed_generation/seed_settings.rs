pub struct SeedSettings {
    pub randomization: RandomizationSettings,
    pub general: GeneralSettings,
    pub qol: QualityOfLifeSettings,
    pub tricks: TrickSettings,
    pub write_spoilerlog: bool,
    pub write_patchfile: bool,
}

pub struct RandomizationSettings {
    pub shuffle_adventure: bool,
    pub shuffle_race_rewards: Option<RewardShuffle>,
    pub warppad_shuffle: Option<WarppadShuffle>,
    pub warppad_unlock_requirements: WarppadUnlockRequirements,
    pub bossgarage_unlock_requirements: BossGarageRequirements,
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

pub struct TrickSettings {
    pub helper_tiziano: bool,
    pub helper_ta: bool,
}

#[derive(Clone, Copy)]
pub struct RewardShuffle {
    pub include_keys: bool,
    pub include_gems: bool,
    pub include_platinum_relics: bool,
}

#[derive(Clone, Copy)]
pub struct WarppadShuffle {
    pub include_battle_arenas: bool,
    pub include_gem_cups: bool,
}

pub enum WarppadUnlockRequirements {
    Vanilla = 0,
    Shuffled = 1,
    Chaotic = 2,
}

impl std::fmt::Display for WarppadUnlockRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            WarppadUnlockRequirements::Vanilla => String::from("Vanilla"),
            WarppadUnlockRequirements::Shuffled => String::from("Shuffled"),
            WarppadUnlockRequirements::Chaotic => String::from("Chaotic"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum BossGarageRequirements {
    Original4Tracks = 0,
    SameHubTracks = 1,
    Trophies = 2,
}

impl std::fmt::Display for BossGarageRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BossGarageRequirements::Original4Tracks => String::from("Original4Tracks"),
            BossGarageRequirements::SameHubTracks => String::from("SameHubTracks"),
            BossGarageRequirements::Trophies => String::from("Trophies"),
        })
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum FinalOxideUnlock {
    SappireRelics18 = 0,
    GoldAndPlatinumRelics18 = 1,
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum RelicTime {
    SapphireTime = 0,
    GoldTime = 1,
    PlatinumTime = 2,
}

impl std::fmt::Display for RelicTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            RelicTime::SapphireTime => String::from("SapphireTime"),
            RelicTime::GoldTime => String::from("GoldTime"),
            RelicTime::PlatinumTime => String::from("PlatinumTime"),
        })
    }
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
            x if x == WarppadUnlockRequirements::Chaotic as i32 => {
                Ok(WarppadUnlockRequirements::Chaotic)
            }
            _ => Err(()),
        }
    }
}

impl TryFrom<i32> for BossGarageRequirements {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == BossGarageRequirements::Original4Tracks as i32 => {
                Ok(BossGarageRequirements::Original4Tracks)
            }
            x if x == BossGarageRequirements::SameHubTracks as i32 => {
                Ok(BossGarageRequirements::SameHubTracks)
            }
            x if x == BossGarageRequirements::Trophies as i32 => {
                Ok(BossGarageRequirements::Trophies)
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

impl std::fmt::Display for FinalOxideUnlock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            FinalOxideUnlock::SappireRelics18 => String::from("SappireRelics18"),
            FinalOxideUnlock::GoldAndPlatinumRelics18 => String::from("GoldAndPlatinumRelics18"),
        })
    }
}
