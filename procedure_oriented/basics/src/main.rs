



fn variable()
{
    println!("variable");

    // 加法
    let sum = 5 + 10;
    println!("{}", sum);

    // 减法
    let difference = 95.5 - 4.3;
    println!("{}", difference);

    // 乘法
    let product = 4 * 30;
    println!("{}", product);

    // 除法
    let quotient = 56.7 / 32.2;
    println!("{}", quotient);

    // 取余
    let remainder = 43 % 5;
    println!("{}", remainder);
}


fn control_flow()
{
    println!("control_flow");

    let a = 5;

    if  a < 5
    {
        println!("a < 5");
    }
    else if  a > 5
    {
        println!("a > 5");
    }
    else if  a == 5
    {
        println!("a == 5");
    }
    else
    {
        println!("error");
    }


    let mut i: i32= 0;
    while i < 5
    {
        println!("i: {}", i);
        i = i + 1;
    }

}

fn array()
{
    println!("array");

    // let a: [i32; 5] = [0; 5];
    let a: [i32; 5] = [0, 1, 2, 3, 4];

    for i in 0..5 // [0, 5)
    // for i in (0..5).rev()
    {   
        println!("a[{}] : {}", i, a[i]);
    }

    for x in a.iter()
    {
        print!("{} ", x);
    }
    println!();
}



fn main() {

    println!("Hello, world!");
    
    variable();

    control_flow();

    array();
    
}
