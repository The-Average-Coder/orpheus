# Orpheus
A mobile app that allows you to turn a tune you hum into an instrumental song sample.

## Overview
Using a genetic algorithm, Orpheus generates and iteratively evolves a chord sequence and groove to fit the melody, finding chords and rhythms that it quantifies as "nice sounding".

## Features
 - An audio recorder to record the hummed melody
 - An autocorrelation algorithm based on YIN to convert this audio into MIDI
 - A genetic algorithm to generate chords and groove
 - A short but high quality list of virtual instruments
 - The ability to export as an audio recording, a MIDI file or chord and melody sheet

## Tech Stack
 - Back end audio processing and music generation in Rust
 - App and UI built for iOS in Swift
