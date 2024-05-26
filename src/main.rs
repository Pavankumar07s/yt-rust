fn main() {
    let mut x:i8=5;
    println!("{}",x);

    for i in 0..100{
        x=x+1;
    }
    print!("x:{}",x+1);
    let greet :String=String::from("hello wold");
    let char1=greet.chars().nth(1);
    println!("{}",char1.unwrap())

}


