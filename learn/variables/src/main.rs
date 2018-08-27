fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
    
    //-------------------------------------

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);

    //-------------------------------------

    let spaces = "   ";
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

     //-------------------------------------

     let _guess: u32 = "42".parse().expect("Not a number!");

     //-------------------------------------

    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

     //-------------------------------------

    let _x = 'z';
    let _x = 'ℤ';
    let _x = '😻';    

    //--------------------------------------

    let _x = true;
    let _x: bool = false;

    //--------------------------------------

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of x, y, z is: {}, {}, {}", x, y, z);

    //--------------------------------------
    //Accessing values in tuple
    //--------------------------------------

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let _six_point_four = x.1;
    let _one = x.2;

    //-------------------------------------
    //Array
    //-------------------------------------

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];
}

