// Generated against hash 83db782 of unofficial_extras_releases

use chrono::{DateTime, Utc};

use crate::{raw_structs::HMODULE, unofficial_extras::raw_structs_keybinds};

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum UserRole {
    SquadLeader = 0,
    Lieutenant  = 1,
    Member      = 2,
    Invited     = 3,
    Applied     = 4,
    None        = 5,
    /// Internal only
    Invalid     = 6,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UserInfoOwned {
    /// Account name, without leading ':'.
    pub account_name: Option<String>,

    /// Unix timestamp when the user joined the squad (or 0 if time could not be
    /// determined)
    pub join_time: u64,

    /// Role in squad, or [`UserRole::None`] if the user was removed from the
    /// squad
    pub role: UserRole,

    /// Subgroup the user is in (0 when no subgroup could be found, which is
    /// either the first subgroup or no subgroup)
    pub subgroup: u8,

    /// Whether this player is ready or not (in a squad ready check).
    /// ### Remarks
    /// `role == `[`UserRole::SquadLeader`] and `ready_status == true` implies
    /// that a ready check was just started. Similarly, `role ==
    /// `[`UserRole::SquadLeader`] and `ready_status == false` implies that
    /// a ready check either finished or was cancelled. If everyone in the
    /// squad had an event sent with `ready_status == true` then that means
    /// that the ready check finished successfully (after which there will
    /// be events sent for each user where their `ready_status == false`)
    pub ready_status: bool,
}

impl From<UserInfo<'_>> for UserInfoOwned {
    fn from(user: UserInfo<'_>) -> Self {
        Self {
            account_name: user.account_name.map(|x| x.to_string()),
            join_time: user.join_time,
            role: user.role,
            subgroup: user.subgroup,
            ready_status: user.ready_status,
        }
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UserInfo<'a> {
    /// Account name, without leading ':'.
    pub account_name: Option<&'a str>,

    /// Unix timestamp when the user joined the squad (or 0 if time could not be
    /// determined)
    pub join_time: u64,

    /// Role in squad, or [`UserRole::None`] if the user was removed from the
    /// squad
    pub role: UserRole,

    /// Subgroup the user is in (0 when no subgroup could be found, which is
    /// either the first subgroup or no subgroup)
    pub subgroup: u8,

    /// Whether this player is ready or not (in a squad ready check).
    /// ### Remarks
    /// `role == `[`UserRole::SquadLeader`] and `ready_status == true` implies
    /// that a ready check was just started. Similarly, `role ==
    /// `[`UserRole::SquadLeader`] and `ready_status == false` implies that
    /// a ready check either finished or was cancelled. If everyone in the
    /// squad had an event sent with `ready_status == true` then that means
    /// that the ready check finished successfully (after which there will
    /// be events sent for each user where their `ready_status == false`)
    pub ready_status: bool,
}

#[repr(C)]
pub struct RawUserInfo {
    /// Null terminated account name, including leading `:`.
    /// Only valid for the duration of the call
    pub account_name: *const u8,

    /// Unix timestamp when the user joined the squad (or 0 if time could not be
    /// determined)
    pub join_time: u64,

    /// Role in squad, or [`UserRole::None`] if the user was removed from the
    /// squad
    pub role: UserRole,

    /// Subgroup the user is in (0 when no subgroup could be found, which is
    /// either the first subgroup or no subgroup)
    pub subgroup: u8,

    /// Whether this player is ready or not (in a squad ready check).
    /// ### Remarks
    /// `role == `[`UserRole::SquadLeader`] and `ready_status == true` implies
    /// that a ready check was just started. Similarly, `role ==
    /// `[`UserRole::SquadLeader`] and `ready_status == false` implies that
    /// a ready check either finished or was cancelled. If everyone in the
    /// squad had an event sent with `ready_status == true` then that means
    /// that the ready check finished successfully (after which there will
    /// be events sent for each user where their `ready_status == false`)
    pub ready_status: bool,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Language {
    English = 0,
    French  = 2,
    German  = 3,
    Spanish = 4,
    Chinese = 5,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub enum ChannelType {
    Party     = 0,
    Squad     = 1,
    _Reserved = 2,
    Invalid   = 3,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SquadMessageInfo<'a> {
    /// A unique identifier for the channel this chat message was sent over. Can
    /// be used to, for example, differentiate between squad messages sent to
    /// different squads
    pub channel_id: u32,

    /// Whether the message is sent in a party or a squad. Note that messages
    /// sent to the party chat while in a squad will have the type
    /// ChannelType::Squad
    pub channel_type: ChannelType,

    /// The subgroup the message was sent to, or UINT8_MAX if it was sent to the
    /// entire squad.
    pub subgroup: u8,

    /// Whether this message is a broadcast or not
    pub is_broadcast: bool,

    /// A DateTime denoting when this message was received by the server. This
    /// is the "absolute ordering" for chat messages, however the time can
    /// potentially differ several seconds between the client and server because
    /// of latency and clock skew.
    // timezone taken from ue code
    pub timestamp: DateTime<Utc>,

    /// Null terminated account name of the player that sent the message,
    /// including leading ':'. The string is only valid for the duration of the
    /// call.
    pub account_name: &'a str,

    /// Null terminated character name of the player that sent the message. The
    /// string is only valid for the duration of the call.
    pub character_name: &'a str,

    /// Null terminated string containing the content of the message that was
    /// sent. The string is only valid for the duration of the call.
    pub text: &'a str,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct NpcMessageInfo<'a> {
    /// Null terminated character name of the NPC or the player character.
    /// The string is only valid for the duration of the call.
    pub character_name: &'a str,

    /// Null terminated string of the message said by an npc or the user
    /// character. The string is only valid for the duration of the call.
    pub message: &'a str,

    /// Time since epoch in nanoseconds.
    /// This can be used to sort messages, when they are out of order.
    // timezone taken from ue code
    pub timestamp: DateTime<Utc>,
}

#[repr(C)]
pub struct RawSquadMessageInfo {
    /// A unique identifier for the channel this chat message was sent over. Can
    /// be used to, for example, differentiate between squad messages sent to
    /// different squads
    pub channel_id: u32,

    /// Whether the message is sent in a party or a squad. Note that messages
    /// sent to the party chat while in a squad will have the type
    /// ChannelType::Squad
    pub channel_type: ChannelType,

    /// The subgroup the message was sent to, or 0 if it was sent to the entire
    /// squad.
    pub subgroup: u8,

    /// This lowest bit of this field will be set to 1 if the message is a
    /// broadcast, and 0 if it is not a broadcast. The upper bits of this field
    /// may be used in a later version and MUST NOT be interpreted
    pub is_broadcast: u8,

    /// Null terminated iso8601 formatted string denoting when this message was
    /// received by the server, e.g. "2022-07-09T11:45:24.888Z". This is the
    /// "absolute ordering" for chat messages, however the time can potentially
    /// differ several seconds between the client and server because of latency
    /// and clock skew. The string is only valid for the duration of the call.
    pub timestamp: *const u8,
    /// does not include the null byte
    pub timestamp_length: u64,

    /// Null terminated account name of the player that sent the message,
    /// including leading ':'. The string is only valid for the duration of the
    /// call.
    pub account_name: *const u8,
    /// does not include the null byte
    pub account_name_length: u64,

    /// Null terminated character name of the player that sent the message. The
    /// string is only valid for the duration of the call.
    pub character_name: *const u8,
    /// does not include the null byte
    pub character_name_length: u64,

    /// Null terminated string containing the content of the message that was
    /// sent. The string is only valid for the duration of the call.
    pub text: *const u8,
    /// does not include the null byte
    pub text_length: u64,
}

#[repr(C)]
pub struct RawNpcMessageInfo {
    /// Null terminated character name of the NPC or the player character.
    /// The string is only valid for the duration of the call.
    pub character_name: *const u8,
    /// does not include the null byte
    pub character_name_length: u64,

    /// Null terminated string of the message said by an npc or the user
    /// character. The string is only valid for the duration of the call.
    pub message: *const u8,
    /// does not include the null byte
    pub message_length: u64,

    /// Time since epoch in nanoseconds.
    /// This can be used to sort messages, when they are out of order.
    pub timestamp: u64,
}

#[repr(C)]
pub struct RawExtrasAddonInfo {
    /// Version of the api, gets incremented whenever a function signature or
    /// behavior changes in a breaking way. Current version is 2.
    pub api_version: u32,

    /// Highest known version of the ExtrasSubscriberInfo struct.
    /// Also determines the size of the pSubscriberInfo buffer in the init call
    /// (the buffer is only guaranteed to have enough space for known
    /// ExtrasSubscriberInfo versions).
    /// Current version is 2.
    pub max_info_version: u32,

    /// String version of unofficial_extras addon, gets changed on every
    /// release. The string is valid for the lifetime of the unofficial_extras
    /// dll.
    pub string_version: *const u8,

    /// Null terminated account name of the logged in player, including leading
    /// ':'. The string is only valid for the duration of the init call.
    pub self_account_name: *const u8,

    /// The handle to the unofficial_extras module.
    /// Use this to call the exports of the DLL.
    pub extras_handle: HMODULE,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[non_exhaustive]
pub enum ChatMessageType {
    // Called for party/squad messages.
    Squad = 0,
    // Called for NPC Channel (selectable in ingame-chat as "NPC")
    NPC   = 1,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[non_exhaustive]
pub enum ChatMessageInfo2<'a> {
    Squad(SquadMessageInfo<'a>),
    Npc(NpcMessageInfo<'a>),
}

#[repr(C)]
pub union RawChatMessageInfo2 {
    pub squad_message_info: *const RawSquadMessageInfo,
    pub npc_message_info: *const RawNpcMessageInfo,
}

pub type RawSquadUpdateCallbackSignature = unsafe extern "C" fn(*const RawUserInfo, u64);
pub type RawLanguageChangedCallbackSignature = unsafe extern "C" fn(Language);
pub type RawKeyBindChangedCallbackSignature =
    unsafe extern "C" fn(raw_structs_keybinds::KeyBindChanged);
pub type RawChatMessageCallbackSignature = unsafe extern "C" fn(*const RawSquadMessageInfo);
pub type RawChatMessage2CallbackSignature =
    unsafe extern "C" fn(ChatMessageType, RawChatMessageInfo2);

#[repr(C)]
pub struct RawExtrasSubscriberInfoHeader {
    /// The version of the following info struct
    /// This has to be set to the version you want to use.
    pub info_version: u32,

    pub _unused1: u32,
}

use std::{
    iter::Map,
    ops::{Deref, DerefMut},
    slice::Iter,
};

pub type ExtrasSquadUpdateCallback = fn(UserInfoIter);
pub type ExtrasChatMessageCallback = fn(&SquadMessageInfo);
pub type ExtrasChatMessage2Callback = fn(ChatMessageInfo2);
pub type UserInfoIter<'a> = Map<Iter<'a, RawUserInfo>, UserConvert>;
pub type UserConvert = for<'r> fn(&'r RawUserInfo) -> UserInfo<'r>;

#[repr(C)]
pub struct RawExtrasSubscriberInfo<T> {
    pub header: RawExtrasSubscriberInfoHeader,
    pub content: T,
}

impl<T> Deref for RawExtrasSubscriberInfo<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.content
    }
}

impl<T> DerefMut for RawExtrasSubscriberInfo<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.content
    }
}

#[repr(C)]
pub struct InfoV1 {
    /// Name of the addon subscribing to the changes. Must be valid for the
    /// lifetime of the subcribing addon. Set to `nullptr` if initialization
    /// fails
    pub subscriber_name: *const u8,

    /// Called whenever anything in the squad changes. Only the users that
    /// changed are sent. If a user is removed from the squad, it will be
    /// sent with [`RawUserInfo::role`]` == `[`UserRole::None`]
    pub squad_update_callback: Option<RawSquadUpdateCallbackSignature>,

    /// Called whenever the language is changed. Either by Changing it in the UI
    /// or by pressing the Right Ctrl (default) key. Will also be called
    /// directly after initialization, with the current language, to get the
    /// startup language.
    pub language_changed_callback: Option<RawLanguageChangedCallbackSignature>,

    /// Called whenever a KeyBind is changed.
    /// By changing it in the ingame UI, by pressing the translation shortcut or
    /// with the Presets feature of this plugin. It is called for every keyBind
    /// separately.
    ///
    /// After initialization this is called for every current keybind that
    /// exists. If you want to get a single keybind, at any time you want, call
    /// the exported function.
    pub key_bind_changed_callback: Option<RawKeyBindChangedCallbackSignature>,
}

#[repr(C)]
pub struct InfoV2 {
    /// contains fields from v1
    pub v1: InfoV1,

    /// Called whenever a chat message is sent in your party/squad
    pub chat_message_callback: Option<RawChatMessageCallbackSignature>,
}

impl Deref for InfoV2 {
    type Target = InfoV1;

    fn deref(&self) -> &Self::Target {
        &self.v1
    }
}

impl DerefMut for InfoV2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v1
    }
}

#[repr(C)]
pub struct InfoV3 {
    /// contains fields from v2
    pub v2: InfoV2,

    /// Called on different chat messages.
    pub chat_message_callback2: Option<RawChatMessage2CallbackSignature>,
}

impl Deref for InfoV3 {
    type Target = InfoV2;

    fn deref(&self) -> &Self::Target {
        &self.v2
    }
}

impl DerefMut for InfoV3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.v2
    }
}

/// This function must be exported by subscriber addons as
/// 'arcdps_unofficial_extras_subscriber_init'. It's called once at startup. Can
/// be called before or after arcdps calls mod_init.
///
/// The callee MUST verify that [`RawExtrasAddonInfo::api_version`] is the
/// version it expects (which is the current api_version when the callee was
/// written). The callee MUST verify that
/// [`RawExtrasAddonInfo::max_info_version`] is equal to or higher
/// than the ExtrasSubscriberInfo struct version it intends to use (to ensure
/// that the buffer has enough room for the info struct). The callee MAY use the
/// [`RawExtrasAddonInfo::max_info_version`] field to dynamically determine
/// which info version to use, in order to gain backwards compatibility. If any
/// of these verifications fail, the callee MUST return without modifying the
/// buffer pointed to by pSubscriberInfo.
///
/// The callee SHOULD populate the buffer pointed to by pSubscriberInfo with one
/// of the ExtrasSubscriberInfo structs above. If initialization fails, the
/// callee SHOULD leave the buffer untouched to indicate initialization failure
pub type RawExtrasSubscriberInitSignature =
    unsafe extern "C" fn(&RawExtrasAddonInfo, &mut RawExtrasSubscriberInfoHeader);

/// Called at startup of unofficial extras. Can be called before or after arcdps
/// init func. Provides the account name and the version of the unofficial
/// extras addon as parameters.
pub type ExtrasInitFunc = fn(Option<&str>, Option<&'static str>);
