use rand::{Rng, rngs::ThreadRng};

const MAX_ROOT_NOTE: u16 = 11;
const MAX_DURATION: u16 = 64;
const MAX_CHORD_TYPE: u16 = 1;

const ROOT_NOTE_MASK: u16 = 0b1111_0_00000;
const ROOT_NOTE_SHIFT: u16 = 6;
const ROOT_NOTE_SIZE: u16 = 4;

const CHORD_TYPE_MASK: u16 = 0b1_00000;
const CHORD_TYPE_SHIFT: u16 = 5;
const CHORD_TYPE_SIZE: u16 = 1;

const DURATION_MASK: u16 = 0b11111;
const DURATION_SHIFT: u16 = 0;
const DURATION_SIZE: u16 = 5;

const ROOT_NOTE_LETTERS: [&str; 12] = ["C", "C♯/D♭", "D", "D♯/E♭", "E", "F", "F♯/G♭", "G", "G♯/A♭", "A", "A♯/B♭", "B"];
const CHORD_TYPE_NAMES: [&str; 2] = ["Major", "Minor"];

pub struct Gene(u16);

impl Gene {

    pub fn get_root_note(&self) -> u16 {
        (self.0 & ROOT_NOTE_MASK) >> ROOT_NOTE_SHIFT
    }

    pub fn set_root_note(&mut self, root_note: u16) {
        if root_note > MAX_ROOT_NOTE {
            return;
        }

        self.0 &= !ROOT_NOTE_MASK;
        self.0 |= root_note << ROOT_NOTE_SHIFT;
    }

    pub fn get_chord_type(&self) -> u16 {
        (self.0 & CHORD_TYPE_MASK) >> CHORD_TYPE_SHIFT
    }

    pub fn set_chord_type(&mut self, chord_type: u16) {
        if chord_type > MAX_CHORD_TYPE {
            return;
        }

        self.0 &= !CHORD_TYPE_MASK;
        self.0 |= chord_type << CHORD_TYPE_SHIFT;
    }

    pub fn get_duration(&self) -> u16 {
        ((self.0 & DURATION_MASK) >> DURATION_SHIFT) + 1
    }

    pub fn set_duration(&mut self, duration: u16) {
        if duration > MAX_DURATION || duration == 0 {
            return;
        }


        self.0 &= !DURATION_MASK;
        self.0 |= (duration - 1) << DURATION_SHIFT;
    }

    pub fn mutate_chord(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {
        if rng.random_bool(mutation_rate) {
            self.set_root_note(rng.random_range(0..(MAX_ROOT_NOTE + 1)));
        }

        if rng.random_bool(mutation_rate) {
            self.set_chord_type(rng.random_range(0..(MAX_CHORD_TYPE + 1)));
        }
    }

    pub fn mutate_duration_and_return_change(&mut self, mutation_rate: f64, rng: &mut ThreadRng) -> i16 {

        let mut duration_mutate_mask = 0;
        for i in 0..DURATION_SIZE {
            let mutate = rng.random_bool(mutation_rate);

            if mutate {
                duration_mutate_mask |= 1 << i;
            }
        }

        // XOR the mutate mask onto the current duration to invert the mutated bits.
        let original_duration = self.get_duration() - 1;
        let mutated_duration = original_duration ^ duration_mutate_mask;
        self.set_duration(mutated_duration + 1);

        mutated_duration as i16 - original_duration as i16
    }

    pub fn print(&self) {
    
        let root_note_letter = ROOT_NOTE_LETTERS[self.get_root_note() as usize];
        let chord_type_name = CHORD_TYPE_NAMES[self.get_chord_type() as usize];
        let chord_duration = self.get_duration();

        println!("{root_note_letter} {chord_type_name} for {chord_duration} 16th beats.");
    }
}

// Factory function to construct a gene with random data.
pub fn generate_random_gene(rng: &mut ThreadRng) -> Gene {
    Gene((rng.random_range(0..12) << ROOT_NOTE_SHIFT) + rng.random_range(0..64))
}