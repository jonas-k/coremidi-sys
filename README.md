# coremidi-sys

Low level Rust bindings for CoreMIDI

`generated.rs` is generated with rust-bindgen 0.26.3 using the following command:
```
bindgen /System/Library/Frameworks/CoreMIDI.framework/Headers/MIDIServices.h --whitelist-type "MIDI.*" --whitelist-function "MIDI.*"  --whitelist-var "kMIDI.*" --no-doc-comments --constified-enum ".*" --blacklist-type "(__)?CF.*" > src/generated.rs
```