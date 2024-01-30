// We can use generics in structs, enums, methods, and functions
#[derive(Debug)]
struct Point<T> {
    _x: T,
    _y: T,
}

// Will only work with f32 numbers
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self._x.powi(2) + self._y.powi(2)).sqrt()
    }
}

// Will work with any type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self._x
    }
}

// We can have multiple generic types
#[derive(Debug)]
struct Point2<T, U> {
    _x: T,
    _y: U,
}

// Returns a Point2 with the x of self and the y of other
// Has to change the types accordingly
impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            _x: self._x,
            _y: other._y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    // Only works with i32 numbers
    let largest = get_largest_number(number_list);
    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];
    // Generic typing
    let largest = get_largest(char_list);
    println!("The largest char is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    // Generic typing
    let largest = get_largest(number_list);
    println!("The largest number is {}", largest);

    let integer = Point { _x: 5, _y: 10 };
    let float = Point { _x: 1.0, _y: 4.0 };
    println!("{:?}", integer);
    println!("{:?}", float);

    // ! let dist = integer.distance_from_origin(); Does not work
    let dist = float.distance_from_origin();
    println!("Distance from origin: {}", dist);

    let x = integer.x();
    println!("x is {}", x);

    // Note that x and y are different types
    let mixed = Point2 { _x: 5, _y: 4.0 };
    println!("{:?}", mixed);

    let mixed = mixed.mixup(Point2 {
        _x: "hello",
        _y: 'c',
    });
    println!("{:?}", mixed);
}

// Not generic
fn get_largest_number(list: Vec<i32>) -> i32 {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number;
        }
    }

    largest
}

// Generic implementation
fn get_largest<T: PartialOrd + Copy>(list: Vec<T>) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
