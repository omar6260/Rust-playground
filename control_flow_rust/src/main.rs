fn main() {
    if_expression();
    handing_multiple_conditions_with_else_if();
    using_if_let_statement();
    // repeating_code_loop();
    returning_values_loop();
    loop_labels_disambiguate_between_multiple_loops();
    condition_loops_with_while();
    looping_through_collection_for();
    exemple_main();
}

fn if_expression(){
    let number = 3;

    if number < 5{
        println!("condition was true");
    }else{
        println!("condition was false");
    }
}

fn handing_multiple_conditions_with_else_if(){
    let number = 7;

    if number % 3 == 0{
        println!("number is divisible by 4");
    } else if number % 3 == 0{
        println!("number is divisible by 3");
    } else if number % 2 == 0{
        println!("number is divisible by 2");
    }else{
        println!("number is not divisible by 4, 3, or 2");
    }
}

fn using_if_let_statement(){
    let condition = true;
    let number = if condition{ 5 } else { 6 };
    println!("the value of number is: {number}");
}

// fn repeating_code_loop(){
//     loop{
//         println!("again!");
//     }
// }

fn returning_values_loop(){
    let mut counter = 0;
    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 2;
        }
    };
    println!("the result is {result}");
}
fn loop_labels_disambiguate_between_multiple_loops() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

// conditional Loops with while

fn condition_loops_with_while(){
    let mut number = 3;

    while number != 0{
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

// Looping Through a Collection with for

fn looping_through_collection_for(){
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}

fn exemple_main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}