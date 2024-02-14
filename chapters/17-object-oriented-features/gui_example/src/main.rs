use gui_example::{Button, Draw, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a select box with width: {}, height: {}, and options: {:?}",
            self.width, self.height, self.options
        );
    }
}

fn main() {
    let select = SelectBox {
        width: 75,
        height: 10,
        options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
        ],
    };

    let button = Button {
        width: 50,
        height: 10,
        label: String::from("OK"),
    };

    let screen = Screen {
        components: vec![Box::new(select), Box::new(button)],
    };

    // Will iterate over the components and call the draw method on each one
    screen.run();
}
