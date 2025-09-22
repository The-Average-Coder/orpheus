mod composer;

const INITIAL_POOL_SIZE: usize = 80;
const SELECTION_POOL_SIZE: usize = 30;
const OFFSPRING_TARGET: usize = 74;
const ELITE_POOL_SIZE: usize = 6;
const NUMBER_OF_GENERATIONS: usize = 1000;

const MUTATION_RATE: f64 = 0.05;

pub fn test() {

    // Melody note representation: (MIDI note code, duration in semiquavers)
    // C - F - - - A F A - - - G - F - - - D - C - - - C - F - - - A F A - - - G - C - - - - - - - - -
    // Melody: Amazing Grace
    let test_melody: Vec::<(u16, u16)> = vec![
        (60, 4), // C for 4 semiquavers
        (65, 8), // F for 8 semiquavers
        (69, 2), // A for 2 semiquavers
        (65, 2), // F for 2 semiquavers
        (69, 8), // A for 8 semiquavers
        (67, 4), // G for 4 semiquavers
        (65, 8), // F for 8 semiquavers
        (62, 4), // D for 4 semiquavers
        (60, 8), // C for 8 semiquavers, change to 4 with rest
        //(0, 4), // Rest for 4 semiquavers
        (60, 4), // C for 4 semiquavers
        (65, 8), // F for 8 semiquavers
        (69, 2), // A for 2 semiquavers
        (65, 2), // F for 2 semiquavers
        (69, 8), // A for 8 semiquavers
        (67, 4), // G for 4 semiquavers
        (72, 20) // C for 20 semiquavers
    ];

    let melody_duration = composer::calculate_melody_duration(&test_melody);

    let crossover_point = melody_duration / 2;

    let mut pool = composer::generate_chromosome_pool(INITIAL_POOL_SIZE, melody_duration);

    for _ in 0..NUMBER_OF_GENERATIONS {

        composer::calculate_pool_fitness(&mut pool, &test_melody);
        let selection_pool = composer::select_top_n_pool(&pool, SELECTION_POOL_SIZE);

        let mut offspring = composer::crossover_selection(&selection_pool, OFFSPRING_TARGET, crossover_point);
        composer::mutate_chromosome_pool(&mut offspring, MUTATION_RATE);

        let elite = composer::select_top_n_pool(&pool, ELITE_POOL_SIZE);

        pool = offspring;
        pool.extend(elite);

    }

    composer::calculate_pool_fitness(&mut pool, &test_melody);
    let top_solutions = composer::select_top_n_pool(&pool, 5);
    composer::print_chromosome_pool(&top_solutions, melody_duration);

}