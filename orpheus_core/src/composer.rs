mod chromosome;

use crate::composer::chromosome::Chromosome;

pub fn generate_chromosome_pool(initial_size: usize, melody_duration: u16) -> Vec::<Chromosome> {

    let mut rng = rand::rng();

    let mut chromosome_pool = Vec::<Chromosome>::new();

    for _ in 0..initial_size {

        let new_chromosome = chromosome::generate_random_chromosome(&mut rng, melody_duration);

        chromosome_pool.push(new_chromosome);

    }

    chromosome_pool

}

pub fn mutate_chromosome_pool(chromosome_pool: &mut Vec::<Chromosome>, mutation_rate: f64) {

    let mut rng = rand::rng();

    for chromosome in chromosome_pool {
        chromosome.mutate(mutation_rate, &mut rng);
    }

}

pub fn print_chromosome_pool(chromosome_pool: &Vec::<Chromosome>, melody_duration: u16) {

    for chromosome in chromosome_pool {
        chromosome.print();
        chromosome.validate(melody_duration);
    }

}