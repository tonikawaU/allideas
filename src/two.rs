pub fn second() {
    let x: i32 = 0;
    println!("x = {x}");
    let x: i32 = x + 2 * 2;
    {
        let x = x * x * x;
        println!("x in the scope is {x}");
    }
    println!("x normaly equals: {x}");
}
