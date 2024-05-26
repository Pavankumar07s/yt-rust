fn main() {
    let mut x:i8=5;
    println!("{}",x);

    for i in 0..100{
        x=x+2;
    }
    print!("x:{}",x+1);
    let greet :String=String::from("hello wold");
    let char1=greet.chars().nth(1);
    println!("{}",char1.unwrap());

    let mut a=32;
    let inc_a =let_a_num(a);

    print!("increamented a is{}",inc_a);
    

}

fn let_a_num( a:i32)->i32{
    
    let a:i32=a+1;
    return a;
}

