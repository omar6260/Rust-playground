fn main() {
    another_function(5);
    print_labeled_measurement(5, 'h');
    print_name('O');
    statement_and_expressions();

    let x = five();
    println!("the value of x is: {x}");

    let y = plus_one(5);

    println!("the value of y is: {y}")
}

fn another_function(x: i32){
    println!("the value is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

fn print_name(full_name: char){
    println!("My name is {full_name}");
}

fn statement_and_expressions(){
    let y = {
        let x = 3;
        x + 1
    };
    println!("the value of y is: {y}")
}

fn five() -> i32{
    5
}

fn plus_one(y: i32) -> i32{
    y + 1
}