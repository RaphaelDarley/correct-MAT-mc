use std::collections::HashMap;

use rand;
use rand::{rngs::ThreadRng, Rng};

fn main() {
    let mut rng = rand::thread_rng();

    let mut correct_acc: Vec<usize> = vec![];
    let mut correct_dist: HashMap<usize, usize> = HashMap::new();

    for _ in 0..1000000 {
        let correct: usize = new_guess(&mut rng)
            .iter()
            .map(|x| (x % 5 == 0) as usize)
            .sum();
        correct_acc.push(correct);
        correct_dist
            .entry(correct)
            .and_modify(|x| *x += 1)
            .or_insert(1);
    }

    let sum: usize = correct_acc.iter().sum();
    let mean: f64 = sum as f64 / correct_acc.len() as f64;

    println!("{:?}", mean);
    println!("{:?}", correct_dist);
}

fn new_guess(rng: &mut ThreadRng) -> Vec<usize> {
    let mut acc = vec![];
    for _ in 0..10 {
        add_new_rand(rng, &mut acc);
    }
    acc
}

fn add_new_rand(rng: &mut ThreadRng, acc: &mut Vec<usize>) {
    let mut new_rand: usize = rng.gen_range(0..51);
    while acc.contains(&new_rand) {
        new_rand = rng.gen_range(0..51);
    }
    acc.push(new_rand);
}
