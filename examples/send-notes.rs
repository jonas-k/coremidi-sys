extern crate core_foundation;
extern crate coremidi_sys;
extern crate time;
extern crate libc;

use core_foundation::string::{CFString, CFStringRef, CFStringGetLength, UniChar};
use core_foundation::base::{TCFType, CFIndex};

use coremidi_sys::{
    MIDIGetNumberOfDestinations, MIDIGetDestination,
    MIDIObjectGetStringProperty, kMIDIPropertyDisplayName,
    MIDIPacketList, MIDIPacket,
    MIDIClientRef, MIDIClientCreate, MIDIClientDispose,
    MIDIPortRef, MIDIOutputPortCreate,
    MIDIEndpointRef, MIDISend, ItemCount };

use std::time::Duration;
use std::thread;
use std::ptr;
use std::mem;
use std::env;

fn main() {
    let destination_index = get_destination_index();

    println!("Destination index: {}", destination_index);

    let client = create_client("example-client");

    let output_port = create_output_port(client, "example-port");

    let endpoint: MIDIEndpointRef = unsafe { MIDIGetDestination(destination_index) };

    for _ in 0..10 {
        println!("Sending note ...");

        let packet_list = create_note_on(0, 64, 127);
        let status = unsafe { MIDISend(output_port, endpoint, &packet_list) };
        assert!(status == 0, "Failed to send a packet list");

        thread::sleep(Duration::from_millis(1000));

        let packet_list = create_note_off(0, 64, 127);
        let status = unsafe { MIDISend(output_port, endpoint, &packet_list) };
        assert!(status == 0, "Failed to send a packet list");
    }

    unsafe { MIDIClientDispose(client) };
}

fn get_destination_index() -> ItemCount {
    let mut args_iter = env::args();
    args_iter.next();
    match args_iter.next() {
        Some(arg) => match arg.parse::<ItemCount>() {
            Ok(index) => {
                let num_devices = unsafe { MIDIGetNumberOfDestinations() };
                if index >= num_devices {
                    println!("Destination index out of range: {}", index);
                    std::process::exit(-1);
                }
                index
            },
            Err(_) => {
                println!("Wrong destination index: {}", arg);
                std::process::exit(-1);
            }
        },
        None => {
            println!("Usage: send <destination-index>");
            println!("");
            println!("Available Outputs:");
            print_destinations();
            std::process::exit(-1);
        }
    }
}

fn print_destinations() {
    unsafe {
        let num_devices = MIDIGetNumberOfDestinations();
        for i in 0..num_devices {
            let endpoint: MIDIEndpointRef = MIDIGetDestination(i);
            match get_display_name(endpoint) {
                Some(display_name) => println!("[{}] {}", i, display_name),
                None => {}
            }
        }
    }
}

fn create_client(name: &str) -> MIDIClientRef {
    let client_name = CFString::new(name);
    let mut client: MIDIClientRef = unsafe { mem::uninitialized() };
    let status = unsafe { MIDIClientCreate(
        client_name.as_concrete_TypeRef(),
        None, ptr::null_mut(),
        &mut client)
    };
    assert!(status == 0, "Failed to create a client");

    client
}

fn create_output_port(client: MIDIClientRef, name: &str) -> MIDIPortRef {
    let output_port_name = CFString::new(name);
    let mut output_port: MIDIPortRef = unsafe { mem::uninitialized() };
    let status = unsafe { MIDIOutputPortCreate(
        client,
        output_port_name.as_concrete_TypeRef(),
        &mut output_port)
    };
    assert!(status == 0, "Failed to create an output port for {}", output_port_name);

    output_port
}

fn create_note_on(channel: u8, note: u8, velocity: u8) -> MIDIPacketList {
    let mut packet = MIDIPacket { timeStamp: 0, length: 3, data: [0; 256] };
    packet.data[0] = 0x90 | (channel & 0x0f);
    packet.data[1] = note & 0x7f;
    packet.data[2] = velocity & 0x7f;

    MIDIPacketList { numPackets: 1, packet: [packet]}
}

fn create_note_off(channel: u8, note: u8, velocity: u8) -> MIDIPacketList {
    let mut packet = MIDIPacket { timeStamp: 0, length: 3, data: [0; 256] };
    packet.data[0] = 0x80 | (channel & 0x0f);
    packet.data[1] = note & 0x7f;
    packet.data[2] = velocity & 0x7f;

    MIDIPacketList { numPackets: 1, packet: [packet] }
}

extern {
    pub fn CFStringGetCharacterAtIndex(theString: CFStringRef, index: CFIndex) -> UniChar;
}

fn get_display_name(endpoint: MIDIEndpointRef) -> Option<String> {
    unsafe {
        let mut display_name_ref: CFStringRef = mem::uninitialized();
        match MIDIObjectGetStringProperty(endpoint, kMIDIPropertyDisplayName, &mut display_name_ref) {
            0 => {
                let mut name_chars = Vec::<char>::new();
                let len = CFStringGetLength(display_name_ref);
                for i in 0..len {
                    let ch = CFStringGetCharacterAtIndex(display_name_ref, i);
                    name_chars.push(std::char::from_u32(ch as u32).unwrap());
                }
                Some(name_chars.iter().cloned().collect::<String>())
            },
            _ => None
        }
    }
}
