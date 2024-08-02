use rand::seq::SliceRandom;
use rand::thread_rng;

fn monty_hall_simulation(switch: bool) -> bool {
    let mut rng = thread_rng();
    let doors = vec![0, 1, 2];
    let prize_door = *doors.choose(&mut rng).unwrap();

    let contestant_choice = *doors.choose(&mut rng).unwrap();

    let doors_to_open = doors
        .iter()
        .filter(|&&door| door != contestant_choice && door != prize_door)
        .cloned()
        .collect::<Vec<_>>();

    let opened_door = *doors_to_open.choose(&mut rng).unwrap();

    let final_choice = if switch {
        *doors
            .iter()
            .find(|&&door| door != contestant_choice && door != opened_door)
            .unwrap()
    } else {
        contestant_choice
    };

    final_choice == prize_door
}

fn main() {
    const SIMULATIONS: u32 = 10000;
    let mut switch_wins = 0;
    let mut non_switch_wins = 0;

    for _ in 0..SIMULATIONS {
        if monty_hall_simulation(true) {
            switch_wins += 1;
        }
        if monty_hall_simulation(false) {
            non_switch_wins += 1;
        }
    }

    println!();
    println!();
    println!("Results:");
    println!(
        "With switching doors, the contestant wins {} out of {} times",
        switch_wins, SIMULATIONS
    );
    println!(
        "Without switching doors, the contestant wins {} out of {} times",
        non_switch_wins, SIMULATIONS
    );
    println!();
    println!();
}
