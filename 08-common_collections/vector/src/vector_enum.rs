#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f32),
    Text(String),
}

impl SpreadsheetCell {
    // A method to explicitly use the fields
    fn print_value(&self) {
        match self {
            SpreadsheetCell::Int(value) => println!("Integer: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }
}

pub fn vector_enum_demo() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // Explicitly use the fields
    for cell in &row {
        cell.print_value();
    }
}