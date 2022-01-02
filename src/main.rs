use rand::rngs::ThreadRng;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

struct Simulation {
    num_door: usize,
    num_iter: usize,
    randomizer: ThreadRng,
}

struct Door {
    num: usize,
    has_prize: bool,
}

impl Simulation {
    fn new(num_door: usize, num_iter: usize) -> Simulation {
        Simulation {
            num_door,
            num_iter,
            randomizer: rand::thread_rng(),
        }
    }

    fn run(&mut self) {
        let mut num = 0;
        let mut den = 0;

        println!(
            "Simulation with {} doors, {} iterations...",
            self.num_door, self.num_iter
        );

        for _ in 0..self.num_iter {
            let mut orig_doors = vec![];
            let prize_door = self.randomizer.gen_range(0..self.num_door);

            // Create doors
            for n in 0..self.num_door {
                orig_doors.push(Door::new(n, n == prize_door))
            }

            // Pick a door!
            let orig_pick = self.randomizer.gen_range(0..self.num_door);

            let orig_door = orig_doors.get(orig_pick).unwrap();
            let othr_door;

            // Eliminate all but one of the other doors
            if orig_pick == prize_door {
                // Pick a door to keep at random
                let mut keep = self.randomizer.gen_range(0..self.num_door - 1);
                if keep >= prize_door {
                    keep += 1;
                }
                othr_door = orig_doors.get(keep).unwrap();
            } else {
                // Pick a door to keep (there's only one valid option)
                othr_door = orig_doors.get(prize_door).unwrap();
            }

            // Strategy: stick to original pick
            let old = if orig_door.has_prize { 1 } else { 0 };

            // Strategy: switch to other door
            let new = if othr_door.has_prize { 1 } else { 0 };

            num += old;
            den += old + new;

            // In case you want to print it out:
            // println!("Where is the prize? Old = {}, New = {}", old, new);
        }

        let res = num as f64 / den as f64;

        println!(
            "Winning likelihood:\n→ Original choice = {}\n→ If you switched = {}",
            res,
            (1.0 - res)
        );
    }
}

impl Door {
    fn new(num: usize, has_prize: bool) -> Door {
        Door { num, has_prize }
    }
}

impl Display for Door {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Door {}", self.num)
    }
}

fn main() {
    println!("{}", "-".repeat(50));
    println!("Monty Hall Simulator");
    println!("{}", "-".repeat(50));
    println!("https://optimix.dev/2022/01/02/monty-hall-problem/");
    println!("{}", "-".repeat(50));
    Simulation::new(3, 1_000_000).run();
}
