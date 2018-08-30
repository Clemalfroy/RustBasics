use std::collections::HashMap;
use std::io;

enum Method {
    Get(String),
    Add {name: String, departement: String},
    Remove {name: String, departement: String},
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

fn build_query(input: &str) -> Option<Method> {

    let words =  input.split_whitespace().enumerate();
    let mut fields = HashMap::new();

    for (i, word) in words { fields.entry(i).or_insert(word); }

    let method = match fields.get(&0) {
        Some(&"add") => Method::Add {
            name: match fields.get(&1){None => return None, Some(x) => x.to_string()},
            departement: match fields.get(&3){None => return None, Some(x) => x.to_string()}
        },
        Some(&"remove") => Method::Remove {
            name: match fields.get(&1){None => return None, Some(x) => x.to_string()},
            departement: match fields.get(&3){None => return None, Some(x) => x.to_string()}
         },
        Some(&"get") => Method::Get(match fields.get(&1){None => return None, Some(x) => x.to_string()}),
        _ => return None,
    };

    Some(method)

}

fn add_to_hash(hashmap: &mut HashMap<String, Vec<String>>, name: String, departement: String) {
    if hashmap.contains_key(&departement) {
        let list_of_names = hashmap.get_mut(&departement);
        match list_of_names {
            Some(vec) => { vec.push(name); },
            None => (),
        }
    } else {
        hashmap.entry(departement).or_insert(vec![name]);
    }
}

fn remove_from_hash(hashmap: &mut HashMap<String, Vec<String>> ,name: String, departement: String) {
    let list_of_names = hashmap.get_mut(&departement);
    match list_of_names {
        Some(vec) => { vec.retain(|x| x != &name) },
        None => (),
    }
}

fn main() {
    
    let mut company = HashMap::new();

    loop {
        let input = user_input();

        let query = build_query(&input);
        if let None = query { continue; }
        let query = query.unwrap();
        
        match query {
            Method::Get(x) => println!("{:?}", company.get(&x)),
            Method::Add{name, departement} => add_to_hash(&mut company, name, departement),
            Method::Remove{name, departement} => remove_from_hash(&mut company, name, departement),
        }

    }
}