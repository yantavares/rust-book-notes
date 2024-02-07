fn main() {
    closures();
    iterators();
}

fn closures() {
    let x = 4;

    let equal_to_x = |z| z == x; // a closure that captures x
                                 // z is a parameter of the closure

    if equal_to_x(4) {
        println!("x is equal to 4");
    }

    let add_1 = |x| x + 1;

    pub fn test(g: impl Fn(i32) -> i32) {
        // test is a function that takes a closure g
        println!("5 + 1 is {}", g(5));
    }

    test(add_1);

    // You can store a closure in a variable, or even a struct
    struct Cacher<T, U>
    where
        T: Fn(U) -> U,
        U: Copy, // This is required because we are storing the value in the struct
    {
        calculation: T,
        value: Option<U>,
    }

    impl<T, U> Cacher<T, U>
    where
        T: Fn(U) -> U,
        U: Copy,
    {
        fn new(calculation: T) -> Cacher<T, U> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: U) -> U {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg); // Parentheses are required to call the closure
                    self.value = Some(v);
                    v
                }
            }
        }
    }
    let mut c = Cacher::new(|a| a);
    let v1 = c.value(1);
    let v2 = c.value(2); // Should return the cached value, not re-calculate

    println!("v1: {}, v2: {}", v1, v2);
}

fn iterators() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {}", val);
    }

    // You can also use the collect method to create a vector from an iterator
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);

    let mut counter = Counter::new();
    for _ in 0..6 {
        match counter.next() {
            Some(n) => println!("Counter: {}", n),
            None => println!("Counter is done"),
        }
    }

    // When we implement the Iterator trait, we get access to all the methods that use the Iterator trait
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();

    println!("Sum: {}", sum);
}

// We can create our own iterator
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    // Iterator that counts from 0 to 5
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

#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    // Passes
}

#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
    // Passes
}

#[test]
fn testing_custom_iterator() {
    let mut counter = Counter::new();

    assert_eq!(counter.count, 0);
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
