use std::{thread, time::Duration};
use terminal_size::{terminal_size, Height, Width};
use std::f64::consts::PI;

const DURATION: Duration = Duration::from_millis(5);
const WORD: &'static str = "Snake";

fn generate_line(n: usize) -> String {
    let mut buf = String::with_capacity(n + WORD.len());
    for _ in 0..n {
        buf.push(' ');
    }
    buf.push_str(WORD);
    buf
}

fn pregenerate_pattern(/*todo: configuration struct*/) -> Vec<String> {
    let mut cycle: f64 = 0.0;
    let (Width(term_width), Height(_)) = terminal_size().expect("Oopsie doopsie something went wrongie!");
    let width = term_width as f64 - WORD.len() as f64;
    let step = PI / width as f64;
    let mut pat = Vec::<String>::new();
    while cycle < PI * 2.0 {
        let space_count = (width * (-cycle.cos() + 1.0) / 2.0).abs() as usize;
        pat.push(generate_line(space_count));
        cycle += step;
    }
    return pat;
}

fn main() {
    let mut i = 0;
    let pat = pregenerate_pattern();
    let pat_len = pat.len();
    loop{
        println!("{}", pat[i]);
        thread::sleep(DURATION);
        i = (i + 1) % pat_len;
    }
}
