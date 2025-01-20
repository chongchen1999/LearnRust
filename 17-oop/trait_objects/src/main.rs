// gui_example.rs

// Define the Draw trait
pub trait Draw {
    fn draw(&self);
}

// Define the Screen struct that holds components implementing the Draw trait
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    // Method to call draw on all components
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Define a Button struct that implements the Draw trait
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!(
            "Drawing a button: {}x{} with label '{}'",
            self.width, self.height, self.label
        );
    }
}

// Define a SelectBox struct that implements the Draw trait
struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!(
            "Drawing a select box: {}x{} with options {:?}",
            self.width, self.height, self.options
        );
    }
}

// Main function to demonstrate the usage
fn main() {
    // Create a Screen instance
    let screen = Screen {
        components: vec![
            // Add a SelectBox
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            // Add a Button
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    // Call the run method to draw all components
    screen.run();
}
