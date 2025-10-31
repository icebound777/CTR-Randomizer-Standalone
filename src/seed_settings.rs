pub struct SeedSettings {
    pub randomization: RandomizationSettings,
    pub general: GeneralSettings,
    pub qol: QualityOfLifeSettings
}

pub struct RandomizationSettings {
    pub shuffle_adventure: bool,
    pub shuffle_race_rewards: bool,
    pub shuffle_warppads: bool,
    pub warppad_unlock_requirements: WarppadUnlockRequirements,
    pub autounlock_ctrchallenge_relicrace: bool
}

pub struct GeneralSettings {
    pub rr_required_minimum_rank: RelicTime,
    pub rr_require_perfects: bool,
    pub oxide_final_challenge_unlock: FinalOxideUnlock
}

pub struct QualityOfLifeSettings {
    pub skip_mask_hints: bool,
    pub autoskip_podium_cutscenes: bool,
    pub skip_mask_congrats: bool
}

pub enum WarppadUnlockRequirements {
    Vanilla = 0,
    Shuffled = 1,
    MoreChoices = 2,
    Wild = 3,
    Chaotic = 4
}

pub enum FinalOxideUnlock {
    SappireRelics18 = 0,
    GoldAndPlatinumRelics18 = 1
}

pub enum RelicTime {
    SapphireTime = 0,
    GoldTime = 1,
    PlatinumTime = 2
}
