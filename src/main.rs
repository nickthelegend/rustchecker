use std::fs::File;
use std::io::{self, BufRead, BufReader};
mod checker;
use checker::checker;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

fn read(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    Ok(lines)
}

fn main() {
    println!("Enter the file name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let filename = input.trim();
 // Clone the value

    println!("Enter the number of threads:");
    let mut input3 = String::new();
    io::stdin().read_line(&mut input3).expect("Failed to read input");
    let num_threads: usize = input3.trim().parse().expect("Invalid input");

    if let Ok(lines) = read(filename) {
        let shared_lines = Arc::new(lines);
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..num_threads {
            let shared_lines = Arc::clone(&shared_lines);
            let counter = Arc::clone(&counter);
 // Clone the value

            let handle = thread::spawn(move || {
                while let Some(line) = shared_lines.get(counter.fetch_add(1, Ordering::Relaxed)) {
                    println!("{}", line);
                    checker(line); // Pass by reference
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("{} ccs found", shared_lines.len());
    } else {
        println!("THERE IS NO SUCH FILE");
    }
}
