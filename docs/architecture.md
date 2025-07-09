# Orpheus Architecture

## System Overview

    +-------------------+
    |    Swift (iOS)    |
    |-------------------|
    | UI + Audio Input  |
    | File Management   |
    | Playback / Export |
    +--------+----------+
                |
                | FFI Bridge (C interface)
                v
    +---------+----------+
    |      Rust Core     |
    |-------------------|
    | Audio Processing  |
    | Melody Analysis   |
    | Genetic Algorithm |
    +-------------------+

## Components

### 1. iOS App (Swift)
- UI Layer: Allows user to record and visualise hummed melody, select from a range of instruments, choose from a small range of chord sequences and grooves the algorithm has generated and playback the current state of the song.
- Audio Layer: Records and saves the audio file before sending it to Rust.
- Bridge Layer: Interfaces with Rust via extern "C" headers and C interop.
- Output: Can export as an audio file, MIDI file or chord and melody sheet. Can also share audio file through social media and group chats.

### 2. Rust Core Library
- Crate: `orpheus_core` built as a staticlib
- FFI Interface: All exposed functions are defined with C-compatible ABI.
- Modules
  - `audio`: Audio preprocessing and pitch detection, converting to MIDI
  - `evolution`: Genetic algorithm to generate chord sequences and grooves

## Data Flow
1. User hums a melody into the app
2. Audio is recorded and saves as a file
3. This is passed onto Rust
4. Autocorrelation algorithm applied to convert this audio into a MIDI file
5. Genetic algorithm takes MIDI file of melody and generates chord sequences
6. Sends choices of chord sequence back to Swift
7. Swift takes choice and turns it into a MIDI file
8. Takes choice of instruments to play it with
9. Handles playback and user export