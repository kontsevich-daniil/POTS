use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct Fork(Mutex<()>);
struct Philosopher {
    id: usize,
    left: Arc<Fork>,
    right: Arc<Fork>,
}

impl Philosopher {
    fn eat(&self) {
        // Решение проблемы deadlock: последний философ берет вилки в другом порядке
        if self.id == 6 { 
            let _r = self.right.0.lock().unwrap();
            let _l = self.left.0.lock().unwrap();
            println!("Философ {} ест", self.id);
        } else {
            let _l = self.left.0.lock().unwrap();
            let _r = self.right.0.lock().unwrap();
            println!("Философ {} ест", self.id);
        }
        thread::sleep(Duration::from_millis(100));
    }

    fn think(&self) {
        println!("Философ {} думает", self.id);
        thread::sleep(Duration::from_millis(100));
    }
}

fn main() {
    let forks = Arc::new((0..7)
        .map(|_| Arc::new(Fork(Mutex::new(()))))
        .collect::<Vec<_>>());

    let mut handles = vec![];

    for i in 0..7 {
        let left = Arc::clone(&forks[i]);
        let right = Arc::clone(&forks[(i + 1) % 5]);

        let phil = Philosopher { id: i, left, right };

        handles.push(thread::spawn(move || {
            for _ in 0..10 {
                phil.think();
                phil.eat();
            }
        }));
    }

    for h in handles { h.join().unwrap(); }
    println!("Все наелись!");
}