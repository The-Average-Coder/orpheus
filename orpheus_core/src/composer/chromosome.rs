pub mod gene;

use rand::rngs::ThreadRng;

use crate::composer::chromosome::gene::{generate_gene, Gene};

pub struct Chromosome(Vec<Gene>);

impl Chromosome {

    fn add_gene(&mut self, gene: Gene) {
        self.0.push(gene);
    }

    pub fn mutate(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {

        self.mutate_chord(mutation_rate, rng);
        self.mutate_duration(mutation_rate, rng);

    }

    fn mutate_chord(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {
        
        for gene in &mut self.0 {

            gene.mutate_chord(mutation_rate, rng);

        }

    }

    fn mutate_duration(&mut self, mutation_rate: f64, rng: &mut ThreadRng) {

        let mut counter: usize = 0;
        while counter < self.0.len() {

            let mutating_gene = &mut self.0[counter];
            let mut duration_change = mutating_gene.mutate_duration_and_return_change(mutation_rate, rng);

            while duration_change != 0 && counter < self.0.len() - 1 {

                let repairing_gene = &mut self.0[counter+1];
                let repairing_gene_duration = repairing_gene.get_duration() as i16;
                
                if repairing_gene_duration > duration_change {
                    repairing_gene.set_duration((repairing_gene_duration - duration_change) as u16);
                    duration_change = 0;
                }
                else {
                    duration_change -= repairing_gene_duration;
                    self.0.remove(counter + 1);
                }
            }
            while duration_change != 0 && counter > 0 {

                let repairing_gene = &mut self.0[counter-1];
                let repairing_gene_duration = repairing_gene.get_duration() as i16;
                
                if repairing_gene_duration > duration_change {
                    repairing_gene.set_duration((repairing_gene_duration - duration_change) as u16);
                    duration_change = 0;
                }
                else {
                    duration_change -= repairing_gene_duration;
                    self.0.remove(counter - 1);
                    counter -= 1;
                }
            }
            if duration_change != 0 {
                println!("Error, not fully fixed duration!")
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
        println!("--------CHROMOSOME--------");
        for gene in &self.0 {
            gene.print();
        }
    }
}

pub fn generate_chromosome(genes: Vec::<Gene>) -> Chromosome {
    Chromosome(genes)
}

// Factory function to construct a chromosome with random genes.
pub fn generate_random_chromosome(rng: &mut ThreadRng, melody_duration: u16) -> Chromosome {

    let mut chromosome = Chromosome(Vec::<Gene>::new());
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