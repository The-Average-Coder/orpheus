mod composer;

const INITIAL_POOL_SIZE: usize = 1;
const MELODY_DURATION: u16 = 4 * 16; // 4 bars
const MUTATION_RATE: f64 = 0.05;

pub fn test() {

    let mut pool = composer::generate_chromosome_pool(INITIAL_POOL_SIZE, MELODY_DURATION);

    composer::print_chromosome_pool(&pool, MELODY_DURATION);

    composer::mutate_chromosome_pool(&mut pool, MUTATION_RATE);

    composer::print_chromosome_pool(&pool, MELODY_DURATION);

}