use super::Condition; 

pub struct Expr {
    valid: bool,
    negative: bool,
    condition: Condition,
}

impl Expr {
    pub fn new(condition: Condition, name: String) -> Expr {
        Expr {
            valid: false,
            negative: false,
            condition: Condition::new(name);
        }
    }

    pub fn make_true(&self) {
        self.condition.true = true;
        self.condition.ambig = false;
        if self.negative == false {
            println!("{} is valid", self.condition.name)
            self.valid = true
        } else {
            println!("{} is not valid", self.condition.name)
            self.valid = false
        }
    }

    pub fn make_false(&self) {
        self.condition.true = false;
        self.condition.ambig = false;
        if self.negative == true {
            println!("{} is valid", self.condition.name)
            self.valid = true;
        } 
        else {
            println!("{} is not valid", self.condition.name)
            self.valid = false;
        }
    }

    pub fn make_ambig(&self) {
        println!("Evaluated {} as ambiguous", self.condition.name);
        self.cond.true = false;
        self.cond.ambig = true;
        self.valid = false;
        println!("{} is invalid", self.cond.name);
    }
}

pub trait Expression {
    pub fn check(&self) {
        for condition in self.expression.condition.condition_list {
            condition.check();
            if condition.expression.valid == true {
                println!("{} is valid inside {}", condition.expression.condition.name, self.expression.condition.name);
                self.expression.make_true();
            } else if condition.expression.valid == false && condition.expression.condition.ambig == false {
                println!("{} is invalid inside {}", condition.expression.condition.name, self.expression.condition.name);
                self.expression.make_false();
                break;
            } else if condition.expression.condition.ambig == true {
                self.expression.make_ambig();
            }
        }
    }
}