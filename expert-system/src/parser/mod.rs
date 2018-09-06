struct Letter(char);

struct Rule(String);

pub struct Parsing {
    queries: Vec<Letter>,
    initial_facts: Vec<Letter>,
    rules: Vec<Rule>,
}

impl Parsing {
    pub fn new(instructions: &str) -> Option<Parsing> {
        let lines = instructions.split('\n');

        let mut queries: Vec<Letter> = Vec::new();
        let mut initial_facts: Vec<Letter> = Vec::new();
        let mut rules: Vec<Rule> = Vec::new();

        for line in lines {
            let line = line.trim_left();
            match line.chars().next() {
                Some('#') | None => continue,
                Some('?') => unimplemented!(),
                Some('=') => unimplemented!(),
                _ => unimplemented!(),
            }
        }
        Some(Parsing{ queries, initial_facts, rules })
    }
}

fn parse_query(line: &str) -> Option<Vec<Letter>> {
    None

}

fn parse_rules(line: &str) -> Option<Vec<Rule>> {
    None
}

fn parse_facts(line: &str)  -> Option<Vec<Letter>> {
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}