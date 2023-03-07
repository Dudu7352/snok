use std::{iter::{repeat, Iterator}, thread, time::Duration};
use terminal_size::{terminal_size, Height, Width};
use std::f64::consts::PI;

const DURATION: Duration = Duration::from_millis(5);
const WORD: &'static str = "Snake";

fn n_snakes(n: usize, space_iter: &mut impl Iterator<Item = char>) -> String {
    let space = space_iter.take(n).collect::<String>();
    format!("{}{}", space, WORD)
}

fn pregenerate_pattern(/*todo: configuration struct*/) -> Vec<String> {
    let mut cycle: f64 = 0.0;
    let size = terminal_size();
    let mut space_iter = repeat(' ');
    let (Width(term_width), Height(_)) = size.expect("Oopsie doopsie something went wrongie!");
    let width = term_width as f64 - WORD.len() as f64;
    let step = PI / width as f64;
    let mut pat = Vec::<String>::new();
    while cycle < PI * 2.0 {
        let space_count = (width * (-cycle.cos() + 1.0) / 2.0).abs() as usize;
        pat.push(n_snakes(space_count, &mut space_iter));
        cycle += step;
    }
    return pat;
}

fn main() {
    pregenerate_pattern().iter().for_each(|el| println!("{}", el));
    thread::sleep(DURATION);
}
