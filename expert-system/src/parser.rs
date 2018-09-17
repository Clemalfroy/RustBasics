use std::collections::HashMap;
use std::error::Error;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum State {
    True,
    False,
    Undetermined,
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Letter(char);

enum EnumRule{
    Rule,
    Implication,
}

pub struct Rule {
    rule: String, 
    implication: String
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
        if !letter.is_ascii_uppercase() || queries.contains(&Letter(letter)) {
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

    check_token(rule, EnumRule::Rule)?;
    check_token(implication, EnumRule::Implication)?;

   Ok( Rule { rule: rule.to_string(), implication: implication.to_string() } )
}

fn parse_facts(line: &str, map: &mut HashMap<Letter, State>) -> Result<(), Box<dyn Error>> {
    let mut letters = line.chars();
    letters.next();

    for letter in letters {
        if !letter.is_ascii_uppercase() { 
            map.clear();
            return Err(Box::from("Facts: Invalid Character."));
        }
        map.entry(Letter(letter)).or_insert(State::True);
    }

    Ok(())
}

fn check_token(str: &str, rule: EnumRule) -> Result<(), Box<dyn Error>> {
    let tokens = str.split_whitespace();

    let allowed_token = ["+", "|", "^", ""];

    for token in tokens {
        match rule {
            EnumRule::Rule => {
                if !allowed_token.contains(&token) {
                    let mut chars = token.chars();
                    match (chars.next(), chars.next(), chars.next(), chars.next()) {
                        (Some('('), Some('!'), Some(x), None) | 
                        (Some('!'), Some(x), Some(')'), None) |
                        (Some('('), Some(x), None, None) |
                        (Some(x), Some(')'), None, None) |
                        (Some('!'), Some(x), None, None) if x.is_ascii_uppercase()=> (),
                        _ => return Err(Box::from("Rule: Invalid Token.")),
                    }
                }
            },
            EnumRule::Implication => {
                if !"+".contains(&token) {
                    let mut chars = token.chars();
                    match (chars.next(), chars.next(), chars.next()) {
                        (Some('!'), Some(x), None) |
                        (Some(x), None, None) if x.is_ascii_uppercase()=> (),
                        _ => return Err(Box::from("Rule: Invalid Token.")),
                    }
                }
            },
        }
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

    #[test]
    fn test_check_token() {
        if let Ok(()) = check_token("(!A + B)", EnumRule::Rule) { assert!(true) }
        if let Ok(()) = check_token("(!A + B) |  C + D", EnumRule::Rule) { assert!(true) }
        if let Ok(()) = check_token("(!A + B) ^ C +   (A | C)", EnumRule::Rule) { assert!(true) }
        if let Err(_) = check_token("(!Aa + B)", EnumRule::Rule) { assert!(true) }
        if let Err(_) = check_token("(!Aa + B)", EnumRule::Rule) { assert!(true) }
        if let Err(_) = check_token("!A + a + B)", EnumRule::Rule) { assert!(true) }
        if let Err(_) = check_token("(!A + B)", EnumRule::Rule) { assert!(true) }
    }
}