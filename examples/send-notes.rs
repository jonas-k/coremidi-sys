extern crate core_foundation;
extern crate coremidi_sys;
extern crate time;
extern crate libc;

use core_foundation::string::{CFString, CFStringRef};
use core_foundation::base::{TCFType};

use coremidi_sys::{
    MIDIGetNumberOfDestinations, MIDIGetDestination,
    MIDIObjectGetStringProperty, kMIDIPropertyDisplayName,
    MIDIPacketList, MIDIPacket,
    MIDIClientRef, MIDIClientCreate, MIDIClientDispose,
    MIDIPortRef, MIDIOutputPortCreate, MIDIPortDispose,
    MIDIEndpointRef, MIDISend, ItemCount, UInt16};

use std::time::Duration;
use std::thread;
use std::ptr;
use std::mem;
use std::env;

fn main() {
    let destination_index = get_destination_index();
    println!("Destination index: {}", destination_index);

    let client = create_client("example-client")
        .expect("Failed to create a client");

    let output_port = create_output_port(client, "example-port")
        .expect("Failed to create an output port");

    let endpoint = get_destination(destination_index);

    for i in 0..10 {
        println!("[{}] Sending note ...", i);

        send(output_port, endpoint, create_note_on(0, 64, 127));

        thread::sleep(Duration::from_millis(1000));

        send(output_port, endpoint, create_note_off(0, 64, 127));
    }

    dispose_port(output_port);
    dispose_client(client);
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

fn get_display_name(endpoint: MIDIEndpointRef) -> Option<String> {
    let mut display_name_ref: CFStringRef = unsafe { mem::uninitialized() };
    let status = unsafe { MIDIObjectGetStringProperty(
        endpoint,
        kMIDIPropertyDisplayName,
        &mut display_name_ref) };
    if status == 0 {
        let display_name: CFString = unsafe { TCFType::wrap_under_create_rule(display_name_ref) };
        Some(format!("{}", display_name))
    }
    else {
        None
    }
}

fn create_client(name: &str) -> Option<MIDIClientRef> {
    let client_name = CFString::new(name);
    let mut client: MIDIClientRef = unsafe { mem::uninitialized() };
    let status = unsafe { MIDIClientCreate(
        client_name.as_concrete_TypeRef(),
        None, ptr::null_mut(),
        &mut client)
    };
    if status == 0 { Some(client) } else { None }
}

fn create_output_port(client: MIDIClientRef, name: &str) -> Option<MIDIPortRef> {
    let output_port_name = CFString::new(name);
    let mut output_port: MIDIPortRef = unsafe { mem::uninitialized() };
    let status = unsafe { MIDIOutputPortCreate(
        client,
        output_port_name.as_concrete_TypeRef(),
        &mut output_port)
    };
    if status == 0 { Some(output_port) } else { None }
}

fn get_destination(index: ItemCount) -> MIDIEndpointRef {
    unsafe { MIDIGetDestination(index) }
}

fn create_note_on(channel: u8, note: u8, velocity: u8) -> Vec<u8> {
    vec![
        0x90 | (channel & 0x0f),
        note & 0x7f,
        velocity & 0x7f]
}

fn create_note_off(channel: u8, note: u8, velocity: u8) -> Vec<u8> {
    vec![
        0x80 | (channel & 0x0f),
        note & 0x7f,
        velocity & 0x7f]
}

fn create_packet_list(data: Vec<u8>) -> MIDIPacketList {
    let len = data.len();
    let mut packet = MIDIPacket { timeStamp: 0, length: len as UInt16, data: [0; 256] };
    packet.data[0..len].clone_from_slice(&data);
    MIDIPacketList { numPackets: 1, packet: [packet]}
}

fn send(port: MIDIPortRef, endpoint: MIDIEndpointRef, data: Vec<u8>) {
    let packet_list = create_packet_list(data);
    let status = unsafe { MIDISend(port, endpoint, &packet_list) };
    assert!(status == 0, "Failed to send data");
}

fn dispose_port(port: MIDIPortRef) {
    unsafe { MIDIPortDispose(port) };
}

fn dispose_client(client: MIDIClientRef) {
    unsafe { MIDIClientDispose(client) };
}
