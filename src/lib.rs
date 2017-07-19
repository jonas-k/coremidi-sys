#![cfg(any(target_os = "macos", target_os = "ios"))]

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

extern crate core_foundation_sys;

use core_foundation_sys::string::*;
use core_foundation_sys::data::*;
use core_foundation_sys::dictionary::*;
use core_foundation_sys::propertylist::*;

include!("generated.rs");

#[inline]
pub unsafe fn MIDIPacketNext(pkt: *const MIDIPacket) -> *const MIDIPacket {
    let ptr = &(*pkt).data as *const u8;
    let offset = (*pkt).length as isize;
    if cfg!(target_arch = "arm") {
        // MIDIPacket must be 4-byte aligned on ARM
        ((ptr.offset(offset + 3) as usize) & !(3usize)) as *const MIDIPacket
    } else {
        ptr.offset(offset) as *const MIDIPacket
    }
}

#[allow(dead_code)]
mod static_test {
    /// Statically assert the correct size of `MIDIPacket` and `MIDIPacketList`,
    /// which require non-default alignment.
    unsafe fn assert_sizes() {
        use super::{MIDIPacket, MIDIPacketList};
        use std::mem::{transmute, zeroed};

        let p: MIDIPacket = zeroed();
        transmute::<_, [u8; 268]>(p);

        let p: MIDIPacketList = zeroed();
        transmute::<_, [u8; 272]>(p);
    }
}