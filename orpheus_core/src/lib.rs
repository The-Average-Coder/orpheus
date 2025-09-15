mod composer;

const INITIAL_POOL_SIZE: usize = 2;
const MELODY_DURATION: u16 = 4 * 16; // 4 bars
const MUTATION_RATE: f64 = 0.05;
const CROSSOVER_POINT: u16 = 2 * 16; // 2 bars

pub fn test() {

    let mut pool = composer::generate_chromosome_pool(INITIAL_POOL_SIZE, MELODY_DURATION);

    composer::print_chromosome_pool(&pool, MELODY_DURATION);

    composer::mutate_chromosome_pool(&mut pool, MUTATION_RATE);

    composer::print_chromosome_pool(&pool, MELODY_DURATION);

    let offspring = composer::crossover_selection(&pool, CROSSOVER_POINT);

    composer::print_chromosome_pool(&offspring, MELODY_DURATION);

}