use std::{iter, thread, time::Duration};
use terminal_size::{terminal_size, Height, Width};

fn main() {
    const PI: f64 = 3.1416;
    let mut step: f64 = 0.0;
    let word = "Snake";
    let size = terminal_size();
    if let Some((Width(term_width), Height(_height))) = size {
        let width = term_width as f64 - word.len() as f64;
        loop {
            let space_count = (width * (step.sin() + 1.0) / 2.0).abs() as usize;
            let space = iter::repeat(" ").take(space_count).collect::<String>();

            println!("{}{}", space, word);

            step += PI / width as f64;
            if step > PI * 2.0 {
                step = step % PI * 2.0;
            }
            thread::sleep(Duration::from_millis(5));
        }
    } else {
        println!("Unable to get terminal size");
    }
}
