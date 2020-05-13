# coremidi-sys

Low level Rust bindings for CoreMIDI

`generated.rs` is generated with [bindgen](https://github.com/rust-lang/rust-bindgen) 0.53.2 using the following command:
```
bindgen /System/Library/Frameworks/CoreMIDI.framework/Headers/MIDIServices.h --whitelist-type "MIDI.*" --whitelist-function "MIDI.*"  --whitelist-var "kMIDI.*" --no-doc-comments --constified-enum ".*" --blacklist-type "(__)?CF.*" > src/generated.rs
```