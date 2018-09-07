use std::collections::HashMap;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum State {
    True,
    False,
    Undetermined,
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Letter(char);

struct Rule {
    rule: String, 
    impliction: String
}

pub struct Parsing {
    queries: Vec<Letter>,
    map_letter: HashMap<Letter, State>,
    rules: Vec<Rule>,
}

impl Parsing {
    pub fn new(instructions: &str) -> Result<Parsing, Box<dyn Error>> {
        let lines = instructions.lines();

        let mut queries = Vec::new();
        let mut map_letter = HashMap::new();
        let mut rules = Vec::new();

        let mut fact_seen = false;

        for line in lines {
            let line = line.trim_left();
            match line.chars().next() {
                Some('#') | None => continue,
                Some('?') => queries.extend(parse_query(line)?),
                Some('=') => {
                        if fact_seen == true { return Err(Box::from("Error : Multiple facts.")) }
                        fact_seen = true;
                        parse_facts(line, &mut map_letter)?;
                    }
                _ => rules.push(parse_rules(line)?),
                }
        }

        if queries.is_empty() || fact_seen == false {
            return Err(Box::from("No query or no fact !"))
        }

        Ok(Parsing{ queries, map_letter, rules })
    }
}

fn parse_query(line: &str) -> Result<Vec<Letter>, Box<dyn Error>> {
    let mut letters = line.chars();
    letters.next();
    
    let mut queries = Vec::new();

    for letter in letters {
        if !letter.is_uppercase() || queries.contains(&Letter(letter)) {
            return Err(Box::from("Query: Not a valid character"));
        }
        queries.push(Letter(letter))
    }

    Ok(queries)
}

fn parse_rules(line: &str) -> Result<Rule, Box<dyn Error>> {
    let mut sides = line.split("=>");

    let left = sides.next();
    let right = sides.next();

    let (rule, implication) = match (left, right, sides.next()) {
        (Some(l), Some(r), None) => (l, r),
        _ => return Err(Box::from("Rule: contains 0 or too many implication(s)"))
    };

    let token_rule = rule.split_whitespace();
    let token_implication = implication.split_whitespace();

   return Err(Box::from("Rule: Invalid Token"))
}

fn parse_facts(line: &str, map: &mut HashMap<Letter, State>) -> Result<(), Box<dyn Error>> {
    let mut letters = line.chars();
    letters.next();

    for letter in letters {
        if !letter.is_uppercase() { 
            map.clear();
            return Err(Box::from("Facts: Invalid Character."));
        }
        map.entry(Letter(letter)).or_insert(State::True);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use parser::*;
    #[test]
    fn test_parse_query() {
        if let Ok(a) = parse_query("?ABC") { assert_eq!(a.len(), 3) }
        if let Err(_) = parse_query("?ABCc") { assert!(true) }
        if let Err(_) = parse_query("?ABCC") { assert!(true) }
        if let Err(_) = parse_query("?ABC?") { assert!(true) }
    }

    #[test]
    fn test_parse_facts() {
        let mut map_letter = HashMap::new();
        if let Ok(()) = parse_facts("=ABC", &mut map_letter){
            assert_eq!(map_letter.len(), 3);
        }

        let mut map_letter = HashMap::new();
        if let Err(_) = parse_facts("=ABCc", &mut map_letter){
            assert_eq!(map_letter.len(), 0);
        }

        let mut map_letter = HashMap::new();
        if let Err(_) = parse_facts("=ABCC", &mut map_letter){
            assert_eq!(map_letter.len(), 3);
        }
    }
}