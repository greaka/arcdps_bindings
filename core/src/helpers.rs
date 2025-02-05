#![allow(clippy::missing_safety_doc)]
use std::ffi::CStr;

use chrono::DateTime;

use crate::*;

/// A helper function to convert raw arguments to safe abstractions
pub unsafe fn get_combat_args_from_raw<'a>(
    raw_ev: Option<&'a CombatEvent>,
    raw_src: Option<&'a RawAgent>,
    raw_dst: Option<&'a RawAgent>,
    raw_skill_name: PCCHAR,
) -> CombatEventArgs<'a> {
    CombatEventArgs {
        ev: raw_ev,
        src: raw_src.map(Into::into),
        dst: raw_dst.map(Into::into),
        skill_name: unsafe { get_str_from_pc_char(raw_skill_name) },
    }
}

/// A helper function to convert arcdps strings to [`&str`].
/// ### Remarks
/// The result is not necessarily static.
/// delta confirmed that skill names are available for the whole lifetime of the
/// plugin, but agent names are only available for the duration of the fight.
/// Reduce the lifetime in the ongoing process as needed!
pub unsafe fn get_str_from_pc_char(src: PCCHAR) -> Option<&'static str> {
    if src.is_null() {
        None
    } else {
        Some(
            CStr::from_ptr(src as *const std::os::raw::c_char)
                .to_str()
                .unwrap_or_default(),
        )
    }
}

/// Converts a pointer and length into a &str with a lifetime. The pointer must
/// not be null
pub unsafe fn get_str_from_ptr_and_len(src: *const u8, len: u64) -> &'static str {
    let buff = std::slice::from_raw_parts(src, len as usize);
    std::str::from_utf8_unchecked(buff)
}

/// A helper function to convert raw arguments to safe abstractions
pub fn convert_extras_user(user: &RawUserInfo) -> UserInfo {
    let name = unsafe { get_str_from_pc_char(user.account_name as _) };
    UserInfo {
        account_name: name.map(|n| n.trim_start_matches(':')),
        join_time: user.join_time,
        role: user.role,
        subgroup: user.subgroup,
        ready_status: user.ready_status,
    }
}

pub fn convert_extras_squad_chat_message(msg: &RawSquadMessageInfo) -> SquadMessageInfo {
    let timestamp = unsafe { get_str_from_ptr_and_len(msg.timestamp, msg.timestamp_length) };
    let timestamp = DateTime::parse_from_rfc3339(timestamp).unwrap().to_utc();

    let account_name =
        unsafe { get_str_from_ptr_and_len(msg.account_name, msg.account_name_length) };
    let character_name =
        unsafe { get_str_from_ptr_and_len(msg.character_name, msg.character_name_length) };
    let text = unsafe { get_str_from_ptr_and_len(msg.text, msg.text_length) };
    let is_broadcast = (msg.is_broadcast & 0x01) != 0;

    SquadMessageInfo {
        channel_id: msg.channel_id,
        channel_type: msg.channel_type,
        subgroup: msg.subgroup,
        is_broadcast,
        timestamp,
        account_name: account_name.trim_start_matches(':'),
        character_name,
        text,
    }
}

pub fn convert_extras_npc_chat_message(msg: &RawNpcMessageInfo) -> NpcMessageInfo {
    let character_name =
        unsafe { get_str_from_ptr_and_len(msg.character_name, msg.character_name_length) };
    let message = unsafe { get_str_from_ptr_and_len(msg.message, msg.message_length) };
    let timestamp = DateTime::from_timestamp_nanos(msg.timestamp as i64);

    NpcMessageInfo {
        character_name,
        message,
        timestamp,
    }
}

pub fn convert_extras_chat_message2<'a>(
    msg_type: ChatMessageType,
    msg: RawChatMessageInfo2,
) -> ChatMessageInfo2<'a> {
    unsafe {
        match msg_type {
            ChatMessageType::Squad => {
                let raw_squad_msg = msg.squad_message_info;
                let squad_info = &*raw_squad_msg;
                let info = convert_extras_squad_chat_message(squad_info);
                ChatMessageInfo2::Squad(info)
            }
            ChatMessageType::NPC => {
                let raw_npc_msg = msg.npc_message_info;
                let npc_info = &*raw_npc_msg;
                let info = convert_extras_npc_chat_message(npc_info);
                ChatMessageInfo2::Npc(info)
            }
        }
    }
}

pub struct CombatEventArgs<'a> {
    pub ev: Option<&'a CombatEvent>,
    pub src: Option<Agent<'a>>,
    pub dst: Option<Agent<'a>>,
    pub skill_name: Option<&'static str>,
}
