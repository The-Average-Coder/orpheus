pub mod gene;
mod precomputed_chord_notes;

use rand::rngs::ThreadRng;
use std::{collections::HashMap, hash::Hash};

use crate::composer::chromosome::gene::{generate_gene, Gene, MAX_DURATION};

// Fitness calculation constants
const CHORD_CONTAINS_MELODY_NOTE_REWARD: i32 = 4;
const CHORD_SCALE_CONTAINS_MELODY_NOTE_REWARD: i32 = 1;
const CHORD_SCALE_DOESNT_CONTAIN_MELODY_NOTE_PENALTY: i32 = 1;

#[derive(Clone)]
pub struct Chromosome(Vec<Gene>, i32);

impl Chromosome {

    fn add_gene(&mut self, gene: Gene) {
        self.0.push(gene);
    }

    pub fn calculate_fitness(&mut self, melody: &Vec::<(u16, u16)>) {

        let mut fitness: i32 = 0;
        
        fitness += self.calculate_chord_melody_fitness(melody);
        fitness += self.calculate_chord_entropy_fitness();
        fitness += self.calculate_duration_entropy_fitness();

        self.1 = fitness;

    }

    fn calculate_chord_melody_fitness(&self, melody: &Vec::<(u16, u16)>) -> i32 {

        let mut fitness: i32 = 0;

        // Go through every melody note.
        // Check whether it is in the chord's note (+3),
        // or in its scale (+1).
        // Penalise if it is neither (-1).

        let mut cumulative_melody_time = 0;
        let mut cumulative_chord_time = 0;
        let mut current_chord_number = 0;

        for note in melody {

            let pitch_class = note.0 % 12;

            cumulative_melody_time += note.1;

            while cumulative_chord_time < cumulative_melody_time {
                current_chord_number += 1;
                cumulative_chord_time += self.0[current_chord_number-1].get_duration();
            }

            if cumulative_chord_time == cumulative_melody_time {
                fitness += 2;
            }

            let chord = self.0[current_chord_number-1].get_chord() as usize;

            if precomputed_chord_notes::CHORD_NOTES[chord].contains(&pitch_class) {
                fitness += CHORD_CONTAINS_MELODY_NOTE_REWARD;
            }
            else if precomputed_chord_notes::CHORD_SCALE_NOTES[chord].contains(&pitch_class) {
                fitness += CHORD_SCALE_CONTAINS_MELODY_NOTE_REWARD;
            }
            else {
                fitness -= CHORD_SCALE_DOESNT_CONTAIN_MELODY_NOTE_PENALTY;
            }

        }

        fitness
    }

    fn calculate_chord_entropy_fitness(&self) -> i32 {

        let chords: Vec<u16> = self.0.iter().map(|c| c.get_chord()).collect();
        let number_of_chords = chords.len();
        
        let mut chord_hash: HashMap<u16, u16> = HashMap::new();

        for chord in &chords {
            match chord_hash.get(&chord) {
                Some(&number) => {chord_hash.insert(*chord, number + 1);},
                _ => {chord_hash.insert(*chord, 1);}
            }
        }

        let mut chord_entropy: f64 = 0.0;
        for (_, value) in chord_hash {
            let probablility: f64 = value as f64 / number_of_chords as f64;
            chord_entropy -= probablility * f64::log2(probablility);
        }

        if 0.8 <= chord_entropy && chord_entropy <= 1.2 {
            return 30;
        }
        else if chord_entropy <= 1.8 || chord_entropy >= 0.6 {
            return 20;
        }
        else if chord_entropy <= 2.4 || chord_entropy >= 0.2 {
            return 10;
        }
        
        -10

    }

    fn calculate_duration_entropy_fitness(&self) -> i32 {
        
        let durations: Vec<u16> = self.0.iter().map(|c| c.get_duration()).collect();
        let number_of_durations = durations.len();

        let mut duration_hash: HashMap<u16, u16> = HashMap::new();

        for duration in durations {
            match duration_hash.get(&duration) {
                Some(&number) => {duration_hash.insert(duration, number + 1);},
                _ => {duration_hash.insert(duration, 1);}
            }
        }

        let mut duration_entropy: f64 = 0.0;
        for (_, value) in duration_hash {
            let probablility: f64 = value as f64 / number_of_durations as f64;
            duration_entropy -= probablility * f64::log2(probablility);
        }

        println!("Duration entropy: {duration_entropy}");

        if 1.8 <= duration_entropy && duration_entropy <= 2.4 {
            return 10;
        }
        else if duration_entropy <= 3.0 || duration_entropy >= 1.2 {
            return 5;
        }   
        
        -10
    }

    pub fn get_fitness(&self) -> i32 {
        self.1
    }

    pub fn mutate(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {

        self.mutate_chord(mutation_rate, rng);
        self.mutate_duration(mutation_rate, rng);

        if !self.is_valid_chromosome(96) {
            println!("Invalid Chromosome!");
        }

    }

    fn mutate_chord(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {
        
        for gene in &mut self.0 {

            gene.mutate_chord(mutation_rate, rng);

        }

    }

    fn mutate_duration(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {

        let mut counter: usize = 0;
        while counter < self.0.len() {

            let mut duration_change = self.0[counter].mutate_duration_and_return_change(mutation_rate, rng);

            let mut repair_counter = counter + 1;

            while duration_change != 0 && repair_counter < self.0.len() {

                let repairing_gene = &mut self.0[repair_counter];
                let repairing_gene_duration = repairing_gene.get_duration() as i16;

                if repairing_gene_duration > duration_change {

                    if (repairing_gene_duration - duration_change) as u16 > gene::MAX_DURATION {
                        repairing_gene.set_duration(MAX_DURATION);
                        duration_change += MAX_DURATION as i16 - repairing_gene_duration;
                        repair_counter += 1;
                    }
                    else {
                        repairing_gene.set_duration((repairing_gene_duration - duration_change) as u16);
                        duration_change = 0;
                    }
                }
                else {
                    duration_change -= repairing_gene_duration;
                    self.0.remove(repair_counter);
                }
            }

            repair_counter = counter;

            while duration_change != 0 && repair_counter > 0 {

                let repairing_gene = &mut self.0[repair_counter-1];
                let repairing_gene_duration = repairing_gene.get_duration() as i16;
                
                if repairing_gene_duration > duration_change {

                    if (repairing_gene_duration - duration_change) as u16 > gene::MAX_DURATION {
                        repairing_gene.set_duration(MAX_DURATION);
                        duration_change += MAX_DURATION as i16 - repairing_gene_duration;
                        repair_counter -= 1;
                    }
                    else {
                        repairing_gene.set_duration((repairing_gene_duration - duration_change) as u16);
                        duration_change = 0;
                    }

                }
                else {
                    duration_change -= repairing_gene_duration;
                    self.0.remove(repair_counter-1);
                    repair_counter -= 1;
                    counter -= 1;
                }
            }
            
            if duration_change != 0 {
                self.0[counter].set_duration(gene::MAX_DURATION);
            }

            counter += 1;

        }
    }

    pub fn split_at_time(&self, time: u16) -> (Vec<Gene>, Vec<Gene>) {
        
        let mut first_segment = Vec::<Gene>::new();
        let mut second_segment = Vec::<Gene>::new();

        let mut first_segment_duration = 0;

        for gene in &self.0 {

            if first_segment_duration >= time {
                second_segment.push(gene.clone());
            }

            // Gene's duration takes segment over the time split, so split the gene.
            else if first_segment_duration + gene.get_duration() > time {
                let first_segment_split_gene_duration = time - first_segment_duration;
                let second_segment_split_gene_duration = gene.get_duration() - first_segment_split_gene_duration;
                
                first_segment.push(generate_gene(gene.get_root_note(), gene.get_chord_type(), first_segment_split_gene_duration));
                second_segment.push(generate_gene(gene.get_root_note(), gene.get_chord_type(), second_segment_split_gene_duration));
            
                first_segment_duration = time;
            }

            else {
                first_segment.push(gene.clone());
                first_segment_duration += gene.get_duration();
            }

        }

        (first_segment, second_segment)
    }

    fn is_valid_chromosome(&self, melody_duration: u16) -> bool {
        let duration: u16 = self.0.iter().map(|gene| gene.get_duration()).sum();

        duration == melody_duration
    }

    pub fn validate(&self, melody_duration: u16) {
        if self.is_valid_chromosome(melody_duration) {
            println!("Valid progression!");
        } else {
            println!("Invalid progression...")
        }
    }

    pub fn print(&self) {
        let fitness = self.1;

        println!("--------CHROMOSOME--------");
        println!("FITNESS: {fitness}");
        for gene in &self.0 {
            gene.print();
        }
    }
}

// Factory function to construct a chromosome with given genes.
pub fn generate_chromosome(genes: Vec::<Gene>) -> Chromosome {
    Chromosome(genes, 0)
}

// Factory function to construct a chromosome with random genes.
pub fn generate_random_chromosome(rng: &mut ThreadRng, melody_duration: u16) -> Chromosome {

    let mut chromosome = Chromosome(Vec::<Gene>::new(), 0);
    let mut duration: u16 = 0;

    // Fill chromosome with genes until melody duration is exceeded.
    while duration < melody_duration {

        let gene = gene::generate_random_gene(rng);

        duration += gene.get_duration();

        chromosome.add_gene(gene);

    }

    // Truncate any overflow of duration.
    let duration_overflow = duration - melody_duration;
    let number_of_genes = chromosome.0.len();
    let final_gene_duration = chromosome.0[number_of_genes - 1].get_duration();
    chromosome.0[number_of_genes - 1].set_duration(final_gene_duration - duration_overflow);

    chromosome
}