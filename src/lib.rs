#![cfg(any(target_os = "macos", target_os = "ios"))]

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

extern crate core_foundation_sys;

use core_foundation_sys::string::*;
use core_foundation_sys::data::*;
use core_foundation_sys::dictionary::*;
use core_foundation_sys::propertylist::*;

use std::ptr;

include!("generated.rs");

#[inline]
pub unsafe fn MIDIPacketNext(pkt: *const MIDIPacket) -> *const MIDIPacket {
    // Get pointer to potentially unaligned data without triggering undefined behavior
    // addr_of does not require creating an intermediate reference to unaligned data.
    let ptr = ptr::addr_of!((*pkt).data);
    let offset = (*pkt).length as isize;
    if cfg!(any(target_arch = "arm", target_arch = "aarch64")) {
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
