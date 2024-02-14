// This implements a simple gui example to show how OO works in Rust

// The Draw trait defines a behavior that will be shared between components
pub trait Draw {
    fn draw(&self);
}

// dyn keyword is short for "dynamic"
// This is a trait object, which allows us to create a collection of different types that implement the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// The run method on Screen will call the draw method on each of its components
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// This works differently than defining a struct that uses a generic type with trait bounds
// because it allows multiple types changing at runtime.
// For example, we can have a Button and a TextField in the same Screen

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Implementing the Draw trait for the Button struct
impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button with width: {}, height: {}, and label: {}",
            self.width, self.height, self.label
        );
    }
}
