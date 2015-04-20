use std::sync::Arc;
use std::vec::Vec;
use std::option::Option;
//use std::thread;

fn main() {
    println!("Hello, world!");
}

struct Entry<T> {
    data: Arc<T>
}
// struct X<T, F: Fn() -> T> { f: F }
struct Pool<T, F: Fn() -> T> {
    available: Vec<Arc<Entry<T>>>,
    in_use: Vec<Arc<Entry<T>>>,
    factory: F,
    size: i32,
    count: i32
}

impl <T, F> Pool<T, F> {
    fn new<F>(size: i32, factory: F)-> Pool<Entry<T>, F>
    where F:Fn() -> T {
        let pool = Pool {
            available: Vec::new(),
            in_use: Vec::new(),
            factory: factory,
            size: size,
            count: 0i32
        };

        for _ in 0..size {
            pool.available.push(factory());
        }
    }

    fn get(&mut self) -> Option<Entry<T>> {

        if(self.count == 0) {
            self.in_use.push(Arc::new(Entry{data:self.factory(), count:1}));
            return Option::Some(self.in_use.last().clone());
        } else {
            return match self.available.pop() {
                    None => {
                        if self.count < self.limit {
                            self.in_use.push(Arc::new(Entry{data:self.factory()}));
                            self.count = self.count + 1;

                            Option::Some(self.in_use.last().clone());
                        }
                    }
                    Some(arc_entry) => {
                        self.in_use.push(arc_entry);
                        Option::Some(self.in_use.last().clone());
                    }
            }
        }

        Option::None
    }
}
