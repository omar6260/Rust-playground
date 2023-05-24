


fn main() {
//    let mut x = 5;
//    println!("the value of x is: {x}");
//    x = 6;
//    println!("the value of x is: {x}");

//    let y = 5;

//    let y = y + 1;

//    {
//     let y = y * 2;
//     println!("the value of y in the inner scope is:{y}");
//    }

//    println!("the value of y is: {y}");

//    variable_exemple();

//    let x:u32 = 10;
//    let y:u32 = 20;

//    println!("Calc: {}", cal_rect(x, y));
//    data_type();
//    floating_point_type();
//    numeric_operations();
   the_character_type();
}

fn variable_exemple(){
    let mut x: i32 = 5;
    println!("le resultat est: {x}");

    x = 7;

    println!("le resultat est: {x}");
}

fn cal_rect(x:u32, y:u32) -> u32
{
    x * y
}

fn data_type(){
    let guess: u32 = "42".parse().expect("Not a number");

    println!("guess: {guess}");
}

fn floating_point_type(){
    let x = 2.0; // f64
    let y: f32 = 3.0; //

    println!("la valeur de x et y : {x} {y}");
}

fn numeric_operations(){
    // addition 
    let sum = 5 + 10;

    println!("sum: {sum}");

    //subtraction
    let difference =  95.5 - 4.3;
    println!("difference: {difference}");

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    println!("quotient: {quotient}");
    println!("truncated {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("remainder: {remainder}");



}

fn the_boolean_type(){
    let t = true;
    let f: bool = false;
}

fn the_character_type(){
    let c = 'z';
    println!("c: {c}");
    let z: char = 'Z';
    println!("z: {z}");
    let heat_eyed_cat = '😻';
    println!("heat_eyed_cat: {heat_eyed_cat}");
}