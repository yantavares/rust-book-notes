// Trait is a way to define shared behavior in an abstract way
pub trait Summary {
    // Traits are exaustive, meaning that all types implementing the trait must implement all the methods
    fn summarize(&self) -> String;
}

pub trait Summary2 {
    // This defines a default implementation for the summarize method
    // Default implementations can be overridden by implementing the method in the type
    fn summarize2(&self) -> String {
        String::from("(Read more...)")
    }
}

pub trait Summary3 {
    fn summarize_author(&self) -> String;

    // This is a default implementation that uses another method from the same trait
    fn summarize3(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

impl Summary3 for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Using the default implementation
impl Summary2 for Tweet {}

// This is a function that takes a type that implements the Summary trait as an argument
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// The previous one was syntactic sugar for this
// This is called a trait bound
pub fn _notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Using trait bounds we can be more specific about the types that we accept
// This function takes two arguments *of the same type* that implement the Summary trait
pub fn _notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {}", item1.summarize());
    println!("Breaking news! {}", item2.summarize());
}

// We can also use the + syntax to accept multiple traits
pub fn _notify3(item: &(impl Summary + Summary3)) {
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item.summarize3());
}

// Same as the previous one, but using trait bounds
pub fn _notify3_2<T: Summary + Summary3>(item: &T) {
    println!("Breaking news! {}", item.summarize());
    println!("Breaking news! {}", item.summarize3());
}

// Sometimes, we need too many trait bounds
fn _some_function<T: std::fmt::Display + Clone, U: Clone + std::fmt::Debug>(_t: &T, _u: &U) -> i32 {
    // hard to read
    0
}

// We can use where clauses to make it more readable
fn _some_function_2<T, U>(_t: &T, _u: &U) -> i32
where
    T: std::fmt::Display + Clone,
    U: Clone + std::fmt::Debug,
{
    0
}

// We can also return a type that implements a trait
// However, we can only return one type at a time
fn _returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    }
}

// Blanket implementations
// We can implement a trait for any type that implements another trait
// This is called a blanket implementation
impl<T: std::fmt::Display> Summary for T {
    // This is a default implementation for any type that implements the Display trait
    fn summarize(&self) -> String {
        format!("{}", self)
    }
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably know, people"),
        reply: false,
        retweet: false,
    };

    println!("Tweet Summary: {}", tweet.summarize());
    println!("Tweet default Summary: {}", tweet.summarize2());
    print!("Using notify function: ");
    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best hockey team in the NHL.",
        ),
    };

    println!("Article Summary: {}", article.summarize());
    println!("Article Summary from author: {}", article.summarize3());
}
