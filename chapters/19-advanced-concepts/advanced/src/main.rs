fn main() {
    advanced_traits();
    macros_example();
}

fn advanced_traits() {
    pub trait Iterator {
        // associated type - will only be known at implementation
        // We can only have one concrete type per implementation
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    struct Counter {
        count: u32,
    }

    impl Iterator for Counter {
        // Item is then defined as u32
        // We cannot have another implementation of Iterator for Counter with a different type
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    let mut counter = Counter { count: 0 };
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
    println!("{:?}", counter.next());
}

fn macros_example() {
    // Creating a simple macro
    macro_rules! say_hello {
        () => {
            println!("Hello!");
        };
    }

    say_hello!();
}

// These notes are shallow, read chapter 19 for more details
