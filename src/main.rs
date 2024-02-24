use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::{thread, time};

use State::*;
use Transition::*;

#[derive(Debug)]
enum Transition {
    Wings,
    Gun,
    Steroids,
    Damage,
}

impl Distribution<Transition> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Transition {
        match rng.gen_range(0..=3) {
            0 => Transition::Wings,
            1 => Transition::Gun,
            2 => Transition::Steroids,
            _ => Transition::Damage,
        }
    }
}

#[derive(Debug)]
enum State {
    Normie,
    Bulked,
    Bomber,
    Winged,
    Dead,
}

struct Player {
    state: State,
}

impl Player {
    fn new() -> Self {
        Self { state: Normie }
    }

    fn collect(&mut self, power: Transition) {
        match (&self.state, power) {
            (Normie, Steroids) => self.state = Bulked,
            (_, Gun) => self.state = Bomber,
            (_, Wings) => self.state = Winged,
            (_, Steroids) => {}
            (Bomber | Winged, Damage) => self.state = Bulked,
            (Bulked, Damage) => self.state = Normie,
            (Normie, Damage) => self.state = Dead,
            _ => {}
        }
    }

    fn print_state(&self) {
        println!("{:?}", self.state);
    }
}

fn main() {
    let mut player: Player = Player::new();

    loop {
        let random_transition: Transition = rand::random();
        player.collect(random_transition);
        player.print_state();
        thread::sleep(time::Duration::from_millis(100));
        match player.state {
            Dead => std::process::exit(0),
            _ => continue,
        }
    }
}
