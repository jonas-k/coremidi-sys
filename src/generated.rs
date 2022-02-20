/* automatically generated by rust-bindgen 0.59.2 */

pub type UInt8 = ::std::os::raw::c_uchar;
pub type UInt16 = ::std::os::raw::c_ushort;
pub type UInt32 = ::std::os::raw::c_uint;
pub type SInt32 = ::std::os::raw::c_int;
pub type UInt64 = ::std::os::raw::c_ulonglong;
pub type OSStatus = SInt32;
pub type ByteCount = ::std::os::raw::c_ulong;
pub type ItemCount = ::std::os::raw::c_ulong;
pub type Boolean = ::std::os::raw::c_uchar;
pub type Byte = UInt8;
pub const kMIDIInvalidClient: ::std::os::raw::c_int = -10830;
pub const kMIDIInvalidPort: ::std::os::raw::c_int = -10831;
pub const kMIDIWrongEndpointType: ::std::os::raw::c_int = -10832;
pub const kMIDINoConnection: ::std::os::raw::c_int = -10833;
pub const kMIDIUnknownEndpoint: ::std::os::raw::c_int = -10834;
pub const kMIDIUnknownProperty: ::std::os::raw::c_int = -10835;
pub const kMIDIWrongPropertyType: ::std::os::raw::c_int = -10836;
pub const kMIDINoCurrentSetup: ::std::os::raw::c_int = -10837;
pub const kMIDIMessageSendErr: ::std::os::raw::c_int = -10838;
pub const kMIDIServerStartErr: ::std::os::raw::c_int = -10839;
pub const kMIDISetupFormatErr: ::std::os::raw::c_int = -10840;
pub const kMIDIWrongThread: ::std::os::raw::c_int = -10841;
pub const kMIDIObjectNotFound: ::std::os::raw::c_int = -10842;
pub const kMIDIIDNotUnique: ::std::os::raw::c_int = -10843;
pub const kMIDINotPermitted: ::std::os::raw::c_int = -10844;
pub const kMIDIUnknownError: ::std::os::raw::c_int = -10845;
pub type _bindgen_ty_63 = ::std::os::raw::c_int;
pub type MIDIObjectRef = UInt32;
pub type MIDIClientRef = MIDIObjectRef;
pub type MIDIPortRef = MIDIObjectRef;
pub type MIDIDeviceRef = MIDIObjectRef;
pub type MIDIEntityRef = MIDIObjectRef;
pub type MIDIEndpointRef = MIDIObjectRef;
pub type MIDITimeStamp = UInt64;
pub type MIDIObjectType = SInt32;
pub const kMIDIObjectType_Other: ::std::os::raw::c_int = -1;
pub const kMIDIObjectType_Device: ::std::os::raw::c_int = 0;
pub const kMIDIObjectType_Entity: ::std::os::raw::c_int = 1;
pub const kMIDIObjectType_Source: ::std::os::raw::c_int = 2;
pub const kMIDIObjectType_Destination: ::std::os::raw::c_int = 3;
pub const kMIDIObjectType_ExternalDevice: ::std::os::raw::c_int = 16;
pub const kMIDIObjectType_ExternalEntity: ::std::os::raw::c_int = 17;
pub const kMIDIObjectType_ExternalSource: ::std::os::raw::c_int = 18;
pub const kMIDIObjectType_ExternalDestination: ::std::os::raw::c_int = 19;
pub type _bindgen_ty_64 = ::std::os::raw::c_int;
pub const kMIDIObjectType_ExternalMask: MIDIObjectType = 16;
pub type MIDIUniqueID = SInt32;
pub const kMIDIInvalidUniqueID: ::std::os::raw::c_uint = 0;
pub type _bindgen_ty_65 = ::std::os::raw::c_uint;
pub type MIDIProtocolID = SInt32;
pub const kMIDIProtocol_1_0: ::std::os::raw::c_uint = 1;
pub const kMIDIProtocol_2_0: ::std::os::raw::c_uint = 2;
pub type _bindgen_ty_66 = ::std::os::raw::c_uint;
pub type MIDINotifyProc = ::std::option::Option<
    unsafe extern "C" fn(message: *const MIDINotification, refCon: *mut ::std::os::raw::c_void),
>;
pub type MIDINotifyBlock = *mut ::std::os::raw::c_void;
pub type MIDIReceiveBlock = *mut ::std::os::raw::c_void;
pub type MIDIReadProc = ::std::option::Option<
    unsafe extern "C" fn(
        pktlist: *const MIDIPacketList,
        readProcRefCon: *mut ::std::os::raw::c_void,
        srcConnRefCon: *mut ::std::os::raw::c_void,
    ),
>;
pub type MIDIReadBlock = *mut ::std::os::raw::c_void;
pub type MIDICompletionProc =
    ::std::option::Option<unsafe extern "C" fn(request: *mut MIDISysexSendRequest)>;
#[repr(C, packed(4))]
#[derive(Debug, Copy, Clone)]
pub struct MIDIEventPacket {
    pub timeStamp: MIDITimeStamp,
    pub wordCount: UInt32,
    pub words: [UInt32; 64usize],
}
#[test]
fn bindgen_test_layout_MIDIEventPacket() {
    assert_eq!(
        ::std::mem::size_of::<MIDIEventPacket>(),
        268usize,
        concat!("Size of: ", stringify!(MIDIEventPacket))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDIEventPacket>(),
        4usize,
        concat!("Alignment of ", stringify!(MIDIEventPacket))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIEventPacket>())).timeStamp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIEventPacket),
            "::",
            stringify!(timeStamp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIEventPacket>())).wordCount as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIEventPacket),
            "::",
            stringify!(wordCount)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIEventPacket>())).words as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIEventPacket),
            "::",
            stringify!(words)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIEventList {
    pub protocol: MIDIProtocolID,
    pub numPackets: UInt32,
    pub packet: [MIDIEventPacket; 1usize],
}
#[test]
fn bindgen_test_layout_MIDIEventList() {
    assert_eq!(
        ::std::mem::size_of::<MIDIEventList>(),
        276usize,
        concat!("Size of: ", stringify!(MIDIEventList))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDIEventList>(),
        4usize,
        concat!("Alignment of ", stringify!(MIDIEventList))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIEventList>())).protocol as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIEventList),
            "::",
            stringify!(protocol)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIEventList>())).numPackets as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIEventList),
            "::",
            stringify!(numPackets)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIEventList>())).packet as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIEventList),
            "::",
            stringify!(packet)
        )
    );
}
#[repr(C, packed(4))]
#[derive(Debug, Clone, Copy)]
pub struct MIDIPacket {
    pub timeStamp: MIDITimeStamp,
    pub length: UInt16,
    pub data: [Byte; 256usize],
}
#[test]
fn bindgen_test_layout_MIDIPacket() {
    assert_eq!(
        ::std::mem::size_of::<MIDIPacket>(),
        268usize,
        concat!("Size of: ", stringify!(MIDIPacket))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDIPacket>(),
        4usize,
        concat!("Alignment of ", stringify!(MIDIPacket))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIPacket>())).timeStamp as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIPacket),
            "::",
            stringify!(timeStamp)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIPacket>())).length as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIPacket),
            "::",
            stringify!(length)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIPacket>())).data as *const _ as usize },
        10usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIPacket),
            "::",
            stringify!(data)
        )
    );
}
#[repr(C)]
#[derive(Debug)]
pub struct MIDIPacketList {
    pub numPackets: UInt32,
    pub packet: [MIDIPacket; 1usize],
}
#[test]
fn bindgen_test_layout_MIDIPacketList() {
    assert_eq!(
        ::std::mem::size_of::<MIDIPacketList>(),
        272usize,
        concat!("Size of: ", stringify!(MIDIPacketList))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDIPacketList>(),
        4usize,
        concat!("Alignment of ", stringify!(MIDIPacketList))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIPacketList>())).numPackets as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIPacketList),
            "::",
            stringify!(numPackets)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDIPacketList>())).packet as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIPacketList),
            "::",
            stringify!(packet)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDISysexSendRequest {
    pub destination: MIDIEndpointRef,
    pub data: *const Byte,
    pub bytesToSend: UInt32,
    pub complete: Boolean,
    pub reserved: [Byte; 3usize],
    pub completionProc: MIDICompletionProc,
    pub completionRefCon: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_MIDISysexSendRequest() {
    assert_eq!(
        ::std::mem::size_of::<MIDISysexSendRequest>(),
        40usize,
        concat!("Size of: ", stringify!(MIDISysexSendRequest))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDISysexSendRequest>(),
        8usize,
        concat!("Alignment of ", stringify!(MIDISysexSendRequest))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDISysexSendRequest>())).destination as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDISysexSendRequest),
            "::",
            stringify!(destination)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDISysexSendRequest>())).data as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDISysexSendRequest),
            "::",
            stringify!(data)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDISysexSendRequest>())).bytesToSend as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDISysexSendRequest),
            "::",
            stringify!(bytesToSend)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDISysexSendRequest>())).complete as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDISysexSendRequest),
            "::",
            stringify!(complete)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDISysexSendRequest>())).reserved as *const _ as usize },
        21usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDISysexSendRequest),
            "::",
            stringify!(reserved)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDISysexSendRequest>())).completionProc as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDISysexSendRequest),
            "::",
            stringify!(completionProc)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDISysexSendRequest>())).completionRefCon as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDISysexSendRequest),
            "::",
            stringify!(completionRefCon)
        )
    );
}
pub type MIDINotificationMessageID = SInt32;
pub const kMIDIMsgSetupChanged: ::std::os::raw::c_uint = 1;
pub const kMIDIMsgObjectAdded: ::std::os::raw::c_uint = 2;
pub const kMIDIMsgObjectRemoved: ::std::os::raw::c_uint = 3;
pub const kMIDIMsgPropertyChanged: ::std::os::raw::c_uint = 4;
pub const kMIDIMsgThruConnectionsChanged: ::std::os::raw::c_uint = 5;
pub const kMIDIMsgSerialPortOwnerChanged: ::std::os::raw::c_uint = 6;
pub const kMIDIMsgIOError: ::std::os::raw::c_uint = 7;
pub type _bindgen_ty_67 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDINotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
}
#[test]
fn bindgen_test_layout_MIDINotification() {
    assert_eq!(
        ::std::mem::size_of::<MIDINotification>(),
        8usize,
        concat!("Size of: ", stringify!(MIDINotification))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDINotification>(),
        4usize,
        concat!("Alignment of ", stringify!(MIDINotification))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDINotification>())).messageID as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDINotification),
            "::",
            stringify!(messageID)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<MIDINotification>())).messageSize as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDINotification),
            "::",
            stringify!(messageSize)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIObjectAddRemoveNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub parent: MIDIObjectRef,
    pub parentType: MIDIObjectType,
    pub child: MIDIObjectRef,
    pub childType: MIDIObjectType,
}
#[test]
fn bindgen_test_layout_MIDIObjectAddRemoveNotification() {
    assert_eq!(
        ::std::mem::size_of::<MIDIObjectAddRemoveNotification>(),
        24usize,
        concat!("Size of: ", stringify!(MIDIObjectAddRemoveNotification))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDIObjectAddRemoveNotification>(),
        4usize,
        concat!("Alignment of ", stringify!(MIDIObjectAddRemoveNotification))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectAddRemoveNotification>())).messageID as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectAddRemoveNotification),
            "::",
            stringify!(messageID)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectAddRemoveNotification>())).messageSize as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectAddRemoveNotification),
            "::",
            stringify!(messageSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectAddRemoveNotification>())).parent as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectAddRemoveNotification),
            "::",
            stringify!(parent)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectAddRemoveNotification>())).parentType as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectAddRemoveNotification),
            "::",
            stringify!(parentType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectAddRemoveNotification>())).child as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectAddRemoveNotification),
            "::",
            stringify!(child)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectAddRemoveNotification>())).childType as *const _
                as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectAddRemoveNotification),
            "::",
            stringify!(childType)
        )
    );
}
#[repr(C)]
pub struct MIDIObjectPropertyChangeNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub object: MIDIObjectRef,
    pub objectType: MIDIObjectType,
    pub propertyName: CFStringRef,
}
#[test]
fn bindgen_test_layout_MIDIObjectPropertyChangeNotification() {
    assert_eq!(
        ::std::mem::size_of::<MIDIObjectPropertyChangeNotification>(),
        24usize,
        concat!(
            "Size of: ",
            stringify!(MIDIObjectPropertyChangeNotification)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<MIDIObjectPropertyChangeNotification>(),
        8usize,
        concat!(
            "Alignment of ",
            stringify!(MIDIObjectPropertyChangeNotification)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectPropertyChangeNotification>())).messageID as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectPropertyChangeNotification),
            "::",
            stringify!(messageID)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectPropertyChangeNotification>())).messageSize as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectPropertyChangeNotification),
            "::",
            stringify!(messageSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectPropertyChangeNotification>())).object as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectPropertyChangeNotification),
            "::",
            stringify!(object)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectPropertyChangeNotification>())).objectType as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectPropertyChangeNotification),
            "::",
            stringify!(objectType)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIObjectPropertyChangeNotification>())).propertyName
                as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIObjectPropertyChangeNotification),
            "::",
            stringify!(propertyName)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct MIDIIOErrorNotification {
    pub messageID: MIDINotificationMessageID,
    pub messageSize: UInt32,
    pub driverDevice: MIDIDeviceRef,
    pub errorCode: OSStatus,
}
#[test]
fn bindgen_test_layout_MIDIIOErrorNotification() {
    assert_eq!(
        ::std::mem::size_of::<MIDIIOErrorNotification>(),
        16usize,
        concat!("Size of: ", stringify!(MIDIIOErrorNotification))
    );
    assert_eq!(
        ::std::mem::align_of::<MIDIIOErrorNotification>(),
        4usize,
        concat!("Alignment of ", stringify!(MIDIIOErrorNotification))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIIOErrorNotification>())).messageID as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIIOErrorNotification),
            "::",
            stringify!(messageID)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIIOErrorNotification>())).messageSize as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIIOErrorNotification),
            "::",
            stringify!(messageSize)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIIOErrorNotification>())).driverDevice as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIIOErrorNotification),
            "::",
            stringify!(driverDevice)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<MIDIIOErrorNotification>())).errorCode as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(MIDIIOErrorNotification),
            "::",
            stringify!(errorCode)
        )
    );
}
extern "C" {
    pub static kMIDIPropertyName: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyManufacturer: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyModel: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyUniqueID: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyDeviceID: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyReceiveChannels: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyTransmitChannels: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyMaxSysExSpeed: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyAdvanceScheduleTimeMuSec: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyIsEmbeddedEntity: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyIsBroadcast: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertySingleRealtimeEntity: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyConnectionUniqueID: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyOffline: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyPrivate: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyDriverOwner: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyFactoryPatchNameFile: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyUserPatchNameFile: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyNameConfiguration: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyNameConfigurationDictionary: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyImage: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyDriverVersion: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertySupportsGeneralMIDI: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertySupportsMMC: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyCanRoute: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyReceivesClock: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyReceivesMTC: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyReceivesNotes: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyReceivesProgramChanges: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyReceivesBankSelectMSB: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyReceivesBankSelectLSB: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyTransmitsClock: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyTransmitsMTC: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyTransmitsNotes: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyTransmitsProgramChanges: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyTransmitsBankSelectMSB: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyTransmitsBankSelectLSB: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyPanDisruptsStereo: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyIsSampler: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyIsDrumMachine: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyIsMixer: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyIsEffectUnit: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyMaxReceiveChannels: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyMaxTransmitChannels: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyDriverDeviceEditorApp: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertySupportsShowControl: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyDisplayName: CFStringRef;
}
extern "C" {
    pub static kMIDIPropertyProtocolID: CFStringRef;
}
extern "C" {
    pub fn MIDIClientCreate(
        name: CFStringRef,
        notifyProc: MIDINotifyProc,
        notifyRefCon: *mut ::std::os::raw::c_void,
        outClient: *mut MIDIClientRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIClientCreateWithBlock(
        name: CFStringRef,
        outClient: *mut MIDIClientRef,
        notifyBlock: MIDINotifyBlock,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIClientDispose(client: MIDIClientRef) -> OSStatus;
}
extern "C" {
    pub fn MIDIInputPortCreateWithProtocol(
        client: MIDIClientRef,
        portName: CFStringRef,
        protocol: MIDIProtocolID,
        outPort: *mut MIDIPortRef,
        receiveBlock: MIDIReceiveBlock,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIInputPortCreate(
        client: MIDIClientRef,
        portName: CFStringRef,
        readProc: MIDIReadProc,
        refCon: *mut ::std::os::raw::c_void,
        outPort: *mut MIDIPortRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIInputPortCreateWithBlock(
        client: MIDIClientRef,
        portName: CFStringRef,
        outPort: *mut MIDIPortRef,
        readBlock: MIDIReadBlock,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIOutputPortCreate(
        client: MIDIClientRef,
        portName: CFStringRef,
        outPort: *mut MIDIPortRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIPortDispose(port: MIDIPortRef) -> OSStatus;
}
extern "C" {
    pub fn MIDIPortConnectSource(
        port: MIDIPortRef,
        source: MIDIEndpointRef,
        connRefCon: *mut ::std::os::raw::c_void,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIPortDisconnectSource(port: MIDIPortRef, source: MIDIEndpointRef) -> OSStatus;
}
extern "C" {
    pub fn MIDIGetNumberOfDevices() -> ItemCount;
}
extern "C" {
    pub fn MIDIGetDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
}
extern "C" {
    pub fn MIDIDeviceGetNumberOfEntities(device: MIDIDeviceRef) -> ItemCount;
}
extern "C" {
    pub fn MIDIDeviceGetEntity(device: MIDIDeviceRef, entityIndex0: ItemCount) -> MIDIEntityRef;
}
extern "C" {
    pub fn MIDIEntityGetNumberOfSources(entity: MIDIEntityRef) -> ItemCount;
}
extern "C" {
    pub fn MIDIEntityGetSource(entity: MIDIEntityRef, sourceIndex0: ItemCount) -> MIDIEndpointRef;
}
extern "C" {
    pub fn MIDIEntityGetNumberOfDestinations(entity: MIDIEntityRef) -> ItemCount;
}
extern "C" {
    pub fn MIDIEntityGetDestination(
        entity: MIDIEntityRef,
        destIndex0: ItemCount,
    ) -> MIDIEndpointRef;
}
extern "C" {
    pub fn MIDIEntityGetDevice(inEntity: MIDIEntityRef, outDevice: *mut MIDIDeviceRef) -> OSStatus;
}
extern "C" {
    pub fn MIDIGetNumberOfSources() -> ItemCount;
}
extern "C" {
    pub fn MIDIGetSource(sourceIndex0: ItemCount) -> MIDIEndpointRef;
}
extern "C" {
    pub fn MIDIGetNumberOfDestinations() -> ItemCount;
}
extern "C" {
    pub fn MIDIGetDestination(destIndex0: ItemCount) -> MIDIEndpointRef;
}
extern "C" {
    pub fn MIDIEndpointGetEntity(
        inEndpoint: MIDIEndpointRef,
        outEntity: *mut MIDIEntityRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIDestinationCreateWithProtocol(
        client: MIDIClientRef,
        name: CFStringRef,
        protocol: MIDIProtocolID,
        outDest: *mut MIDIEndpointRef,
        readBlock: MIDIReceiveBlock,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIDestinationCreate(
        client: MIDIClientRef,
        name: CFStringRef,
        readProc: MIDIReadProc,
        refCon: *mut ::std::os::raw::c_void,
        outDest: *mut MIDIEndpointRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIDestinationCreateWithBlock(
        client: MIDIClientRef,
        name: CFStringRef,
        outDest: *mut MIDIEndpointRef,
        readBlock: MIDIReadBlock,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDISourceCreateWithProtocol(
        client: MIDIClientRef,
        name: CFStringRef,
        protocol: MIDIProtocolID,
        outSrc: *mut MIDIEndpointRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDISourceCreate(
        client: MIDIClientRef,
        name: CFStringRef,
        outSrc: *mut MIDIEndpointRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIEndpointDispose(endpt: MIDIEndpointRef) -> OSStatus;
}
extern "C" {
    pub fn MIDIGetNumberOfExternalDevices() -> ItemCount;
}
extern "C" {
    pub fn MIDIGetExternalDevice(deviceIndex0: ItemCount) -> MIDIDeviceRef;
}
extern "C" {
    pub fn MIDIObjectGetIntegerProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outValue: *mut SInt32,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectSetIntegerProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        value: SInt32,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectGetStringProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        str_: *mut CFStringRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectSetStringProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        str_: CFStringRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectGetDataProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outData: *mut CFDataRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectSetDataProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        data: CFDataRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectGetDictionaryProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        outDict: *mut CFDictionaryRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectSetDictionaryProperty(
        obj: MIDIObjectRef,
        propertyID: CFStringRef,
        dict: CFDictionaryRef,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectGetProperties(
        obj: MIDIObjectRef,
        outProperties: *mut CFPropertyListRef,
        deep: Boolean,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectRemoveProperty(obj: MIDIObjectRef, propertyID: CFStringRef) -> OSStatus;
}
extern "C" {
    pub fn MIDIObjectFindByUniqueID(
        inUniqueID: MIDIUniqueID,
        outObject: *mut MIDIObjectRef,
        outObjectType: *mut MIDIObjectType,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDISendEventList(
        port: MIDIPortRef,
        dest: MIDIEndpointRef,
        evtlist: *const MIDIEventList,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDISend(
        port: MIDIPortRef,
        dest: MIDIEndpointRef,
        pktlist: *const MIDIPacketList,
    ) -> OSStatus;
}
extern "C" {
    pub fn MIDISendSysex(request: *mut MIDISysexSendRequest) -> OSStatus;
}
extern "C" {
    pub fn MIDIReceivedEventList(src: MIDIEndpointRef, evtlist: *const MIDIEventList) -> OSStatus;
}
extern "C" {
    pub fn MIDIReceived(src: MIDIEndpointRef, pktlist: *const MIDIPacketList) -> OSStatus;
}
extern "C" {
    pub fn MIDIFlushOutput(dest: MIDIEndpointRef) -> OSStatus;
}
extern "C" {
    pub fn MIDIRestart() -> OSStatus;
}
extern "C" {
    pub fn MIDIEventListInit(
        evtlist: *mut MIDIEventList,
        protocol: MIDIProtocolID,
    ) -> *mut MIDIEventPacket;
}
extern "C" {
    pub fn MIDIEventListAdd(
        evtlist: *mut MIDIEventList,
        listSize: ByteCount,
        curPacket: *mut MIDIEventPacket,
        time: MIDITimeStamp,
        wordCount: ByteCount,
        words: *const UInt32,
    ) -> *mut MIDIEventPacket;
}
extern "C" {
    pub fn MIDIPacketListInit(pktlist: *mut MIDIPacketList) -> *mut MIDIPacket;
}
extern "C" {
    pub fn MIDIPacketListAdd(
        pktlist: *mut MIDIPacketList,
        listSize: ByteCount,
        curPacket: *mut MIDIPacket,
        time: MIDITimeStamp,
        nData: ByteCount,
        data: *const Byte,
    ) -> *mut MIDIPacket;
}
