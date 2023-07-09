use std::thread;
fn main() {
    let handle = thread::spawn(|| {
        // Code to be executed in the thread
        println!("Hello from a separate thread!");
    });

    // Main thread continues executing here
    println!("Hello from the main thread!");

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}