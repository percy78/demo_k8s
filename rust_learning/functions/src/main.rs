fn main() {
    println!("Hello, world!");

    another_function(5);
    and_another_function();
    print_labeled_measurement(5, 'h');

    let y = {
        let x = 3;
        x + 6 / 3 - 2 * 8
    };

    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("Another function. {}", x);
}

fn and_another_function() {
    println!("And another function.");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
    let b = five();

println!("The value of b is: {b}");
}