pub mod chromosome;

use rand::{rngs::ThreadRng, seq::IndexedRandom};

use crate::composer::chromosome::Chromosome;

pub fn calculate_melody_duration(melody: &Vec::<(u16, u16)>) -> u16 {
    melody.iter().map(|note| note.1).sum()
}

pub fn calculate_pool_fitness(chromosome_pool: &mut Vec::<Chromosome>, melody: &Vec::<(u16, u16)>) {

    for chromosome in chromosome_pool {
        chromosome.calculate_fitness(melody);
    }

}

pub fn select_top_n_pool(chromosome_pool: &Vec::<Chromosome>, quantity: usize) -> Vec::<Chromosome> {

    if quantity >= chromosome_pool.len() {
        return chromosome_pool.clone();
    }

    let mut sorted_chromosome_pool: Vec::<Chromosome> = chromosome_pool.clone();
    sorted_chromosome_pool.sort_by(|c1, c2| c2.get_fitness().cmp(&c1.get_fitness()));

    let mut top_n = sorted_chromosome_pool[0..quantity].to_vec();
    
    top_n.reverse();

    top_n

}

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

pub fn crossover_selection(selection: &Vec::<Chromosome>, target: usize, crossover_point: u16) -> Vec::<Chromosome> {

    let mut rng = rand::rng();

    let mut offspring = Vec::<Chromosome>::new();

    while offspring.len() < target {

        let (parent_one_index, parent_two_index) = choose_two_parent_indices(&mut rng, selection);

        let parent_one = &selection[parent_one_index];
        let parent_two = &selection[parent_two_index];
        let (child_one, child_two) = crossover_parents(parent_one, parent_two, crossover_point);

        offspring.extend([child_one, child_two]);

    }

    offspring

}

fn crossover_parents(parent_one: &Chromosome, parent_two: &Chromosome, crossover_point: u16) -> (Chromosome, Chromosome) {
    
    let (parent_one_first_segment, parent_one_second_segment) = parent_one.split_at_time(crossover_point);
    let (parent_two_first_segment, parent_two_second_segment) = parent_two.split_at_time(crossover_point);

    let mut child_one_genes = parent_one_first_segment;
    child_one_genes.extend(parent_two_second_segment);
    let child_one = chromosome::generate_chromosome(child_one_genes);

    let mut child_two_genes = parent_two_first_segment;
    child_two_genes.extend(parent_one_second_segment);
    let child_two = chromosome::generate_chromosome(child_two_genes);

    (child_one, child_two)

}

fn choose_two_parent_indices(rng: &mut ThreadRng, parent_pool: &Vec::<Chromosome>) -> (usize, usize) {
    let indices: Vec<usize> = (0..parent_pool.len()).collect();
    let parent_indices: Vec<usize> = indices.choose_multiple(rng, 2).copied().collect();
    (parent_indices[0], parent_indices[1])
}

pub fn print_chromosome_pool(chromosome_pool: &Vec::<Chromosome>, melody_duration: u16) {

    for chromosome in chromosome_pool {
        chromosome.print();
        chromosome.validate(melody_duration);
    }

}