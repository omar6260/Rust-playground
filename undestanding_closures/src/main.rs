fn main() {
    basic_closure();
    let one = 2;
    let closure = move |int_input|{
        return int_input * one
    };

    let outcome = add_doubles(Box::new(closure), 2,3);
    println!("{}", outcome);
    basic_closures();

}

fn basic_closure(){
    let test_closure = |string_input|{
        println!("{}", string_input);
    };
    test_closure("test");
}

fn add_doubles(closure: Box<dyn Fn(i32) -> i32>,
                one: i32, two:i32) -> i32{
    return closure(one) + closure(two)
    
}

fn basic_closures(){
    let launch_closur= |string_input|{
        println!("{}", string_input);
    
    };
    launch_closur("test");
}