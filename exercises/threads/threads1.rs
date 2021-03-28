// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // Need a Mutex to allow for freezing data access. It shouldn't be accessed concurrently.
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    // Clone the Arc to provide a reference that's used only in the thread (?)
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            // Acquire the lock on status_shared, then get the reference to the value inside.
            // It's mutable, so I think it tracks both the reference and the value to the reference -
            // allowing access to one while mutating the other? I like that Rust does this for you
            // but the machinery here feels a bit inconsistent. I like small tight rules sets.
            let mut status_shared = status_shared.lock().unwrap();
            status_shared.jobs_completed += 1;
        }
    });

    let mut jobs_completed = status.lock().unwrap().jobs_completed;
    while jobs_completed < 10 {
        // Because status_shared shares the mutex, we have to snag the lock
        // to track the counting here too.
        jobs_completed = status.lock().unwrap().jobs_completed;
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }
}
