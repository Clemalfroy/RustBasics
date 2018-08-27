fn main() {
    let mut s = String::from("hello world");

    let _word = first_word(&s); // word will get the value 5

    s.clear(); // This empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that
    // we could meaningfully use the value 5 with. word is now totally invalid!

    //------------------------------------------------------

    let s = String::from("hello world");
    let _hello = &s[0..5];
    let _world = &s[6..11];

    let s = String::from("hello");
    let len = s.len();
    let _slice = &s[0..len];
    let _slice = &s[..];

    //--------------------------------------------------------

    let my_string = String::from("hello world");
    // first_word works on slices of `String`s
    let _word = first_wordd(&my_string[..]);
    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let _word = first_wordd(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_wordd(my_string_literal);

    //---------------------------------------------------------

    let a = [1, 2, 3, 4, 5];
    let _slice = &a[1..3];
    
}

// Good way

fn first_wordd(s: &str) -> &str { // &str means slice
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// Bad Way

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}