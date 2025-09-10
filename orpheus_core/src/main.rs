use core::num;

use rand::prelude::*;

// Chord representation
// 4 bits for root note (12 possible notes)
// 3 bits for major, minor, [diminished, augmented, sus2, sus4]?
// 1 bit for a major seventh?
// 1 bit for a minor seventh?
// 2 bits for inversion?
// 5 bits for duration
// minimum 10 bits, maximum 16 bits

const MELODY_DURATION: u16 = 8 * 16; // 8 bars

const MAX_DURATION: u16 = 2 << 5 - 1;

const ROOT_NOTE_MASK: u16 = 0b1111_0_00000;
const ROOT_NOTE_SHIFT: u16 = 6;

const CHORD_TYPE_MASK: u16 = 0b1_00000;
const CHORD_TYPE_SHIFT: u16 = 5;

const DURATION_MASK: u16 = 0b11111;
const DURATION_SHIFT: u16 = 0;

const ROOT_NOTE_LETTERS: [&str; 12] = ["C", "C♯/D♭", "D", "D♯/E♭", "E", "F", "F♯/G♭", "G", "G♯/A♭", "A", "A♯/B♭", "B"];
const CHORD_TYPE_NAMES: [&str; 2] = ["Major", "Minor"];


struct Gene(u16);

impl Gene {
    fn get_root_note(&self) -> u16 {
        (&self.0 & ROOT_NOTE_MASK) >> ROOT_NOTE_SHIFT
    }

    fn get_chord_type(&self) -> u16 {
        (&self.0 & CHORD_TYPE_MASK) >> CHORD_TYPE_SHIFT
    }

    fn get_duration(&self) -> u16 {
        (&self.0 & DURATION_MASK) >> DURATION_SHIFT
    }

    fn set_duration(&mut self, duration: u16) {
        if duration > MAX_DURATION {
            return;
        }

        self.0 |= DURATION_MASK;
        self.0 &= duration << DURATION_SHIFT;
    }

    fn print(&self) {
    
        let root_note_letter = ROOT_NOTE_LETTERS[self.get_root_note() as usize];
        let chord_type_name = CHORD_TYPE_NAMES[self.get_chord_type() as usize];
        let chord_duration = self.get_duration();

        println!("{root_note_letter} {chord_type_name} for {chord_duration} 16th beats.");
    }
}


fn generate_chromosome(melody_duration: u16) -> Vec<Gene> {
    let mut chromosome: Vec<Gene> = Vec::new();
    let mut duration: u16 = 0;

    let mut rng = rand::rng();

    // Fill chromosome with genes until melody duration is exceeded.
    while duration < melody_duration {

        let gene = Gene((rng.random_range(0..12) << ROOT_NOTE_SHIFT) + rng.random_range(0..64));

        duration += gene.get_duration();

        chromosome.push(gene);

    }

    // Truncate any overflow of duration.
    let duration_overflow = duration - melody_duration;
    let number_of_genes = chromosome.len();
    let final_gene_duration = chromosome[number_of_genes - 1].get_duration();
    chromosome[number_of_genes - 1].set_duration(final_gene_duration - duration_overflow);

    chromosome
}

fn validate_chromosome(chromosome: Vec<Gene>, melody_duration: u16) -> bool {
    let duration: u16 = chromosome.iter().map(|gene| gene.get_duration()).sum();

    duration == melody_duration
}

fn main() {
    let test_chromosome = generate_chromosome(MELODY_DURATION);

    for gene in &test_chromosome {
        gene.print();
    }

    if validate_chromosome(test_chromosome, MELODY_DURATION) {
        println!("Valid progression!");
    } else {
        println!("Invalid progression...")
    }
}