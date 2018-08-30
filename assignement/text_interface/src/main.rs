use std::collections::HashMap;
use std::io;

struct Query {
    method: Method,
    departement: String,
    name: String,
}

enum Method {
    Add,
    Remove,
    Get,
}

fn user_input() -> String {
        let mut input = String::new();

        println!("Enter your demand : ");
        io::stdin().read_line(&mut input)
            .expect("Failed to read from stdin");
        let len = input.len();
        input.truncate(len - 1);
        
        input
}

fn build_query(input: &str) -> Option<Query> {

    let words =  input.split_whitespace().enumerate();

    let mut fields = HashMap::new();

    for (i, word) in words { fields.entry(i).or_insert(word); }

    let method = match fields.get(&0) {
        Some(&"add") => Method::Add,
        Some(&"remove") => Method::Remove,
        Some(&"get") => Method::Get,
        _ => return None,
    };
    match method {
        Method::Add | Method::Remove => {
            match fields.get(&2) {
                Some(&"to") => (),
                _ => return None,
            };
        }
        _ => (),
    }
    let departement = match method {
        Method::Add | Method::Remove => {
            match fields.get(&3) {
                Some(&x) => x,
                _ => return None,
            }
        }
        Method::Get => {
            match fields.get(&1) {
                Some(&x) => x,
                _ => return None,
            }
        }
    };
    let name = match method {
        Method::Add | Method::Remove => {
            match fields.get(&1) {
                Some(x) => x,
                _ => return None,
            }
        }
        Method::Get => "",
    };
    Some(Query { method, departement: departement.to_string(), name: name.to_string() })
}

fn main() {

    let mut company = HashMap::new();

    loop {

        let input = user_input();
        if input == "exit" { break; }
        
        let mut query = build_query(&input).unwrap();

        match query.method {
            Method::Add => { company.entry(query.departement).or_insert(query.name); },
            Method::Get => { println!("get: {:?}:", company.get(&query.departement)); },
            Method::Remove => { company.remove(&query.departement); },
        }
    }

}
