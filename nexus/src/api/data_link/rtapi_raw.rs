// Bindings for RTAPI.h
use std::ffi::c_char;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum EGameState {
    CharacterSelection,
    CharacterCreation,
    Cinematic,
    LoadingScreen,
    Gameplay,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum EGameLanguage {
    English,
    Korean,
    French,
    German,
    Spanish,
    Chinese,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ETimeOfDay {
    Dawn,
    Day,
    Dusk,
    Night,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum ECharacterState {
    IsAlive = 1 << 0,
    IsDowned = 1 << 1,
    IsInCombat = 1 << 2,
    IsSwimming = 1 << 3,   // aka. Is on water surface
    IsUnderwater = 1 << 4, // aka. Is diving
    IsGliding = 1 << 5,
    IsFlying = 1 << 6,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum EMapType {
    AutoRedirect,
    CharacterCreation,
    PvP,
    GvG,
    Instance,
    Public,
    Tournament,
    Tutorial,
    UserTournament,
    WvWEternalBattlegrounds,
    WvWBlueBorderlands,
    WvWGreenBorderlands,
    WvWRedBorderlands,
    WVWFortunesVale,
    WvWObsidianSanctum,
    WvWEdgeOfTheMists,
    PublicMini,
    BigBattle,
    WvWLounge,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum EGroupType {
    None,
    Party,
    RaidSquad,
    Squad,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct GroupMember {
    pub account_name: [c_char; 140],   // Account name of the group member
    pub character_name: [c_char; 140], // Character name of the group member
    pub subgroup: i32,                 // 0 for parties, 1-15 according to the squad's subgroup
    pub profession: i32, // 0-9 = Profession; -1 Unknown -> e.g. on loading screen or logged out
    pub elite_specialization: i32, // Third Spec ID, not necessarily elite; or -1 Unknown -> e.g. on loading screen or logged out
    pub is_self: u32,              // Is this member the player themselves?
    pub is_in_instance: u32,       // Is in the same map instance as the player.
    pub is_commander: u32,         // Is this member a commander?
    pub is_lieutenant: u32,        // Is this member a lieutenant?
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct RealTimeData {
    /* Game Data */
    pub game_build: i32, // Game build number
    pub game_state: u32, // Current game state
    pub language: u32,   // Language setting

    /* Instance/World Data */
    pub time_of_day: u32,             // Time of day
    pub map_id: i32,                  // Current map ID
    pub map_type: u32,                // Type of the map
    pub ip_address: [u8; 4],          // IP address (4 bytes)
    pub cursor: [f32; 3],             // Location of cursor in the world
    pub squad_markers: [[f32; 3]; 8], // Locations of squad markers
    pub group_type: u32,              // Type of group
    pub group_member_count: u32,      // Number of group members
    pub reserved2: u32,               // Reserved for future use

    /* Player Data */
    _reserved1: u32,                   // Reserved for future use
    pub account_name: [c_char; 140],   // Player's account name
    pub character_name: [c_char; 140], // Player's character name
    pub character_position: [f32; 3],  // Player's character position
    pub character_facing: [f32; 3],    // Player's character facing direction
    pub profession: i32,               // Player's profession
    pub elite_specialization: i32,     // Player's elite specialization
    pub mount_index: i32,              // Index of the mount, if applicable
    pub character_state: u32,          // Current state of the character

    /* Camera Data */
    pub camera_position: [f32; 3], // Camera position in the world
    pub camera_facing: [f32; 3],   // Camera facing direction
    pub camera_fov: f32,           // Camera field of view
    pub is_action_camera: u32,     // Is this an action camera?
}

// Constants
pub const RTAPI_SIG: u32 = 0x2501A02C;
pub const DL_RTAPI: &str = "RTAPI";
pub const EV_RTAPI_GROUP_MEMBER_JOINED: &str = "RTAPI_GROUP_MEMBER_JOINED";
pub const EV_RTAPI_GROUP_MEMBER_LEFT: &str = "RTAPI_GROUP_MEMBER_LEFT";
pub const EV_RTAPI_GROUP_MEMBER_UPDATED: &str = "RTAPI_GROUP_MEMBER_UPDATED";
