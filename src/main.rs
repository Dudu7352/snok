use std::{thread, time::Duration};
use terminal_size::{terminal_size, Height, Width};
use std::f64::consts::PI;

mod config;

const DURATION: Duration = Duration::from_millis(5);
const WORD: &'static str = "Snake";
const ERROR_MSG: &'static str = "Oopsie doopsie something went wrongie!";

fn generate_line(n: usize) -> String {
    let mut buf = String::with_capacity(n + WORD.len());
    for _ in 0..n {
        buf.push(' ');
    }
    buf.push_str(WORD);
    buf
}

fn pregenerate_pattern(conf: config::Config) -> Vec<String> {
    let mut cycle: f64 = 0.0;
    let width = match conf.width {
        config::SizeConfig::Percent(p) => {
            let (Width(term_width), Height(_)) = terminal_size().expect(ERROR_MSG);
            term_width as f64 * p/100.0 - WORD.len() as f64
        },
        config::SizeConfig::Chars(n) => (n as usize - WORD.len()) as f64,
    };
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
    let conf = config::read_config("snok.pson".to_string()).expect(ERROR_MSG);
    let pat = pregenerate_pattern(conf);
    let pat_len = pat.len();
    let mut i = 0;
    loop{
        println!("{}", pat[i]);
        thread::sleep(DURATION);
        i = (i + 1) % pat_len;
    }
}
