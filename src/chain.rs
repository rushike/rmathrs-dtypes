#![allow(dead_code)]

use std::{collections::HashMap, any::{TypeId, Any}, fmt::Debug};
use num_traits::Num;

#[derive(Debug)]
enum Operator {
    Int32,
    Float32,
    Int64,
    Float64,

    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Sqrt,
    Cbrt,
    Nroot,
    
    Sum,
    Prod
}

#[derive(Debug)]
pub enum Numeric {
    Int32 (i32),
    Int64 (i64),
    Float32 (f32),
    Float64 (f64),
    None
}

#[derive(Debug)]
enum Operand {
    Val (Numeric)
}

#[derive(Debug)]
enum Symbol {
    Oprt (Operator),
    Oprd (Operand)
}

pub enum Expr{
    Int32 (i32),
    Int64 (i64),
    Float32 (f32),
    Float64 (f64),
    LocalChain
}

pub trait Action {
    fn calc(&self);
}


pub trait Operations {
    fn add<T : Num +  Debug + Copy + PartialOrd>(self, a : T) -> Self;
}

pub trait Init {
    fn var(self, name : &str) -> Self;
    fn int<T : Num +  Debug + Copy + PartialOrd + 'static>(self, a : T) -> Self;
    // fn numeric<T :  Num +  Debug + Copy + PartialOrd + 'static>(&self, a : T) -> Numeric;
}

#[derive(Debug)]
pub struct  LocalChain {
    expr : Vec<Symbol>
}


#[derive(Debug)]
pub struct Chain {
    root : HashMap<String, LocalChain>,
    varstack : Vec<String>
}

trait Basic<T :  Num +  Debug + Copy + PartialOrd> {
    fn numeric(a : T) -> Numeric;
}

impl Basic<i32> for Numeric {
    fn numeric(a : i32) -> Numeric {
        return Numeric::Int32(a);
    }
}

impl Basic<f32> for Numeric {
    fn numeric(a : f32) -> Numeric {
        return Numeric::Float32(a);
    }
}


impl Init for Chain {
    fn var(mut self, name : &str) -> Self{
        let localchain = LocalChain {expr : Vec::new()};
        self.root.insert(String::from(name), localchain);
        self.varstack.push(String::from(name));
        return  self;
    }

    fn int <T :  Num +  Debug + Copy + PartialOrd + 'static> (mut self, a : T) -> Self {
        let currvar = self.varstack.last().unwrap();
        let currchain = self.root.get_mut(currvar).unwrap();
        // let operand = Symbol::Oprd(Operand::Val());
        // currchain.expr.push(operand);
        // self.numeric(a);
        println!("Numeric : : : {:?}", Numeric::numeric(a));
        return self;
    }

   
}


impl Operations for Chain {
    fn add<T :  Num +  Debug + Copy + PartialOrd >(mut self, a : T) -> Self {
        let currvar = self.varstack.last().unwrap();
        let currchain = self.root.get_mut(currvar).unwrap();
        // let operand = Symbol::Oprd(Operand::Val(a));
        let operator = Symbol::Oprt(Operator::Add);
        // currchain.expr.push(operand);
        currchain.expr.push(operator);
        return self;
    }    
}

impl Action for Chain{
    fn calc(&self) {
            
    }
}

pub fn fast_i32() -> Chain {
    return Chain {
        root : HashMap::new(), 
        varstack : Vec::new()
    };
}

pub fn fast_f32() -> Chain{
    return Chain {
        root : HashMap::new(), 
        varstack : Vec::new()
    };
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tree() {
        let root = fast_i32();
        let test = root.
        var("test")
        .int(4)
        .add(9);
        println!("I am printing : {:?}", test)
    }

}