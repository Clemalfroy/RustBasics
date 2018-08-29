fn main() {
    
    let mut s = String::new();

    let data = "initial contents";
    let _s = data.to_string();
    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    //------------------------------------------------------

    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    //------------------------------------------------------

    let mut s = String::from("foo");
    s.push_str("bar");
    let mut s = String::from("lo");
    s.push('l');

    //-----------------------------------------------------

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used

    //---------------------------------------------------

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = s1 + "-" + &s2 + "-" + &s3;

    //---------------------------------------------------

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{}-{}-{}", s1, s2, s3);

    //--------------------------------------------------

    let len = String::from("Hola").len();
    let len = String::from("Здравствуйте").len();

    //--------------------------------------------------

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    //-------------------------------------------------

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}