


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

   variable_exemple();

   let x:u32 = 10;
   let y:u32 = 20;

   println!("Calc: {}", cal_rect(x, y));
   data_type();
   floating_point_type();
}

fn variable_exemple(){
    let mut x = 5;
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