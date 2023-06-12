fn main() {
    binding_and_mutability();
    shadowing_exemple();
}


// Binding and mutability
fn binding_and_mutability(){
    let mut x = 1;
    x += 2;

    assert_eq!(x, 3);
    println!("Succes")
}

//Scope

// Shadowing

fn shadowing_exemple(){
    let x: i32 = 5;
    {
        let x = 12;
        assert_eq!(x, 5);
    }

    assert_eq!(x, 12);
    println!("{}", x);
}
