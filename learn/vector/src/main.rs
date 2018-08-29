// https://doc.rust-lang.org/std/vec/struct.Vec.html

fn main() {

    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    //----------------------------------------------

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // Panic if index out of range
    let third: Option<&i32> = v.get(2); // Value None if out of range

    //-----------------------------------------------

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    //-----------------------------------------------

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
