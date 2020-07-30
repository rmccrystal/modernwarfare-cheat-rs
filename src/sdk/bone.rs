use memlib::math::Vector3;
use memlib::memory::{read_memory, Address};
use log::*;
use super::{offsets};
use crate::sdk::Game;

#[repr(u32)]
#[derive(Clone, Copy, Debug)]
pub enum Bone {
    Helmet = 8,
    Head = 7,
    Neck = 6,
    Chest = 5,
    Mid = 4,
    Tummy = 3,
    Pelvis = 2,

    RightFoot1 = 21,
    RightFoot2 = 22,
    RightFoot3 = 23,
    RightFoot4 = 24,

    LeftFoot1 = 17,
    LeftFoot2 = 18,
    LeftFoot3 = 19,
    LeftFoot4 = 20,

    LeftHand1 = 13,
    LeftHand2 = 14,
    LeftHand3 = 15,
    LeftHand4 = 16,

    RightHand1 = 9,
    RightHand2 = 10,
    RightHand3 = 11,
    RightHand4 = 12
}

pub fn get_bone_position(game: &Game, entity_num: i32, bone_index: u32) -> Result<Vector3, Box<dyn std::error::Error>> {
    let bone_ptr_index: u16 = read_memory(game.base_address + offsets::INDEX_ARRAY + (entity_num as u64 * std::mem::size_of::<u16>() as u64));
    trace!("bone_ptr_index: 0x{:X}", bone_ptr_index);

    let bone_base = game.get_bone_base().ok_or("Could not find bone_base")?;

    let bone_ptr: Address = read_memory(bone_base + (bone_ptr_index as u64 * 0x150) + 0xC0);
    if bone_ptr == 0 {
        return Err("Could not find bone_ptr".into());
    }
    trace!("Found bone_ptr: 0x{:X}", bone_ptr);

    let bone_pos: Vector3 = read_memory(bone_ptr as u64 + (bone_index as u64 * 0x20) + 0x10);
    if bone_pos.is_zero() {
        return Err("Could not find bone_pos".into());
    }

    let client_info = game.get_client_info_base().ok_or("Could not find client_info_base")?;
    let base_pos: Vector3 = read_memory(client_info + offsets::bones::BASE_POS);

    Ok(bone_pos + base_pos)
}