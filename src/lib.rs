#![cfg(any(target_os = "macos", target_os = "ios"))]

#![allow(non_snake_case, non_upper_case_globals, non_camel_case_types)]

extern crate core_foundation_sys;

use core_foundation_sys::string::*;
use core_foundation_sys::data::*;
use core_foundation_sys::dictionary::*;
use core_foundation_sys::propertylist::*;

use std::{ptr, mem};

include!("generated.rs");

#[inline]
pub unsafe fn MIDIPacketNext(pkt: *const MIDIPacket) -> *const MIDIPacket {
    // Get pointer to potentially unaligned data without triggering undefined behavior
    // addr_of does not require creating an intermediate reference to unaligned data.
    // See also the definition of `MIDIPacketNext` in the official SDK MIDIServices.h
    let ptr = ptr::addr_of!((*pkt).data) as *const u8;
    let ptr_length = ptr::addr_of!((*pkt).length) as *const u16;
    if cfg!(any(target_arch = "arm", target_arch = "aarch64")) {
        // MIDIPacket must be 4-byte aligned on ARM, so we need to calculate an aligned offset.
        // We do not need `read_unaligned` for the length, because the length will never
        // be unaligned, and `read_unaligned` would lead to less efficient machine code.
        let offset = ptr_length.read() as isize;
        ((ptr.offset(offset + 3) as usize) & !(3usize)) as *const MIDIPacket
    } else {
        // MIDIPacket is unaligned on non-ARM, so reading the length requires `read_unaligned`
        // to not trigger Rust's UB check (although unaligned reads are harmless on Intel
        // and `read_unaligned` will generate the same machine code as `read`).
        let offset = ptr_length.read_unaligned() as isize;
        ptr.offset(offset) as *const MIDIPacket
    }
}

#[inline]
pub unsafe fn MIDIEventPacketNext(pkt: *const MIDIEventPacket) -> *const MIDIEventPacket {
    // Each EventPacket's size is a multiple of 4 bytes, so no special care
    // needs to be taken when reading the data (except the timeStamp, which is not 8-byte aligned).
    // See also the definition of `MIDIEventPacketNext` in the official SDK MIDIServices.h
    let ptr = ptr::addr_of!((*pkt).words) as *const u8;
    let offset = (((*pkt).wordCount as usize) * mem::size_of::<u32>()) as isize;
    ptr.offset(offset) as *const MIDIEventPacket
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midi_packet_next() {
        const BUFFER_SIZE: usize = 65536;
        let buffer: &mut [u8] = &mut [0; BUFFER_SIZE];
        let pkt_list_ptr = buffer.as_mut_ptr() as *mut MIDIPacketList;

        let packets = vec![
            (1, vec![0x90, 0x40, 0x7f]), // tuple of (time, [midi bytes])
            (2, vec![0x90, 0x41, 0x7f]),
        ];

        unsafe {
            let mut pkt_ptr = MIDIPacketListInit(pkt_list_ptr);
            for pkt in &packets {
                pkt_ptr = MIDIPacketListAdd(
                    pkt_list_ptr,
                    BUFFER_SIZE as ByteCount,
                    pkt_ptr,
                    pkt.0,
                    pkt.1.len() as ByteCount,
                    pkt.1.as_ptr(),
                );
                assert!(!pkt_ptr.is_null());
            }
        }

        unsafe {
            let first_packet = &(*pkt_list_ptr).packet as *const MIDIPacket; // get pointer to first midi packet in the list
            let len = (*first_packet).length as usize;
            assert_eq!(
                &(*first_packet).data[0..len],
                &[0x90, 0x40, 0x7f]
            );

            let second_packet = MIDIPacketNext(first_packet);
            let ptr_length = ptr::addr_of!((*second_packet).length) as *const u16;
            let len = ptr_length.read_unaligned() as usize;
            assert_eq!(
                &(*second_packet).data[0..len],
                &[0x90, 0x41, 0x7f]
            );
        }
    }

    #[test]
    fn midi_event_packet_next() {
        const BUFFER_SIZE: usize = 65536;
        let buffer: &mut [u8] = &mut [0; BUFFER_SIZE];
        let pkt_list_ptr = buffer.as_mut_ptr() as *mut MIDIEventList;

        let packets = vec![
            (1, vec![10u32, 20]), // tuple of (time, [midi words])
            (2, vec![30u32, 40, 50]),
        ];

        unsafe {
            let mut pkt_ptr = MIDIEventListInit(pkt_list_ptr, kMIDIProtocol_2_0 as MIDIProtocolID);
            for pkt in &packets {
                pkt_ptr = MIDIEventListAdd(
                    pkt_list_ptr,
                    BUFFER_SIZE as ByteCount,
                    pkt_ptr,
                    pkt.0,
                    pkt.1.len() as ByteCount,
                    pkt.1.as_ptr(),
                );
                assert!(!pkt_ptr.is_null());
            }
        }

        unsafe {
            let first_packet = &(*pkt_list_ptr).packet as *const MIDIEventPacket; // get pointer to first midi packet in the list
            let len = (*first_packet).wordCount as usize;
            assert_eq!(
                &(*first_packet).words[0..len],
                &[10, 20]
            );

            let second_packet = MIDIEventPacketNext(first_packet);
            let len = (*second_packet).wordCount as usize;
            assert_eq!(
                &(*second_packet).words[0..len],
                &[30, 40, 50]
            );
        }
    }
}
