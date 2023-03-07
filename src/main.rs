use std::{iter, thread, time::Duration};
use terminal_size::{terminal_size, Height, Width};

const DURATION: Duration = Duration::from_millis(5);
const PI: f64 = 3.1416;
const WORD: &'static str = "Snake";

fn main() {
    let mut cycle: f64 = 0.0;
    let size = terminal_size();
    let mut space_iter = iter::repeat(" ");
    let (Width(term_width), Height(_)) = size.expect("Oopsie doopsie something went wrongie!");
    let width = term_width as f64 - WORD.len() as f64;
    let step = PI / width as f64;
    loop {
        let space_count = (width * (-cycle.cos() + 1.0) / 2.0).abs() as usize;
        let space = (&mut space_iter).take(space_count).collect::<String>();

        println!("{}{}", space, WORD);

        cycle += step;
        if cycle > PI * 2.0 {
            cycle = cycle %  (PI * 2.0);
        }
        thread::sleep(DURATION);
    }
}
