use std::error::Error;
use std::fs;

mod parser;

pub fn get_filename(mut args: std::env::Args) -> Result<String, &'static str> {
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a query string"),
    };
    Ok(filename)
}

pub fn run(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let parsed = parser::Parsing::new(&contents);






    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}