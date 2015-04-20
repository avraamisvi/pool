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

impl <T, F: Fn() -> T> Pool<T, F> {

    pub fn new(size: i32, factory: F)-> Pool<T, F> {
        //let pool =
        Pool {
            available: Vec::new(),
            in_use: Vec::new(),
            factory: factory,
            size: size,
            count: 0i32
        }

        /*for _ in 0..size {
            pool.available.push(Entry{data:factory()});
        }*/
    }

    fn call_closure<C: Fn()->T>(&mut self, c1: &C)->Arc<T> {
        Arc::new(c1())
    }

    fn increase(&mut self) {
        self.count = self.count + 1;
    }

    pub fn get(&self) -> Option<&Arc<Entry<T>>> {

        if(self.count == 0) {
            let entry = Entry{data: self.call_closure(&self.factory)};
            self.in_use.push(Arc::new(entry));

            return self.in_use.last().clone();
        } else {
            return match self.available.pop() {
                    None => {
                        if self.count < self.size {

                            let entry = Entry{data:self.call_closure(&self.factory)};

                            self.in_use.push(Arc::new(entry));

                            self.increase();

                            self.in_use.last().clone()
                        } else {
                            Option::None
                        }
                    }
                    Some(arc_entry) => {
                        self.in_use.push(arc_entry);

                        self.in_use.last().clone()
                    }
            }
        }

        Option::None
    }
}
