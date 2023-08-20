use std::fmt::{Display, Formatter};

use crate::grammar::ast::{
    expr::{ComputeOp, MathOp},
    Type,
};

use super::ast2ir::Dest;

#[derive(Debug, Clone)]
pub struct AbstractProgram {
    pub functions: Vec<AbstractFunction>,
}

impl Display for AbstractProgram {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for func in &self.functions {
            writeln!(f, "{}", func)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AbstractFunction {
    pub args: Vec<AbstractArgument>,
    pub blocks: Vec<AbstractBasicBlock>,
    pub name: String,
    pub return_type: AbstractType,
}

impl Display for AbstractFunction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}", self.name)?;
        if !self.args.is_empty() {
            write!(f, "(")?;
            for (i, arg) in self.args.iter().enumerate() {
                if i != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", arg)?;
            }
            write!(f, ")")?;
        }
        // if let Some(tpe) = self.return_type {
        write!(f, ": {}", self.return_type)?;

        writeln!(f, " {{")?;
        for block in &self.blocks {
            writeln!(f, "{}", block)?;
        }
        write!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AbstractBasicBlock {
    pub instrs: Vec<AbstractCode>,
    pub name: String,
    pub successors: Vec<Self>,
}

impl Display for AbstractBasicBlock {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.name)?;
        for instr in &self.instrs {
            writeln!(f, "{}", instr)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct AbstractArgument {
    pub name: String,
    pub arg_type: AbstractType,
}

impl Display for AbstractArgument {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.arg_type)
    }
}

#[derive(Debug, Clone)]
pub enum AbstractCode {
    Label { label: String },
    Instruction(AbstractInstruction),
}

impl Display for AbstractCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AbstractCode::Label { label } => write!(f, ".{}:", label),
            AbstractCode::Instruction(instr) => write!(f, "  {}", instr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum AbstractInstruction {
    Constant {
        dest: String,
        const_type: AbstractType,
        value: Literal,
    },
    Compute {
        left: String,
        dest: Dest,
        right: String,
        op: MathOp,
        type_: Type,
    },
    Assign {
        src: String,
        dest: Dest,
        type_: Option<AbstractType>,
    },

    Ret {
        src: String,
    },

    Value {
        args: Vec<String>,
        dest: String,
        funcs: Vec<String>,
        labels: Vec<String>,
        src: String,
        type_: Option<AbstractType>,
    },
    Effect {
        args: Vec<String>,
        funcs: Vec<String>,
        labels: Vec<String>,
        op: String,
    },
}

impl Display for AbstractInstruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AbstractInstruction::Constant {
                dest,
                const_type,
                value,
            } => {
                write!(f, "{}: {} = const {};", dest, const_type, value)
            }
            AbstractInstruction::Compute {
                left,
                dest,
                right,
                op,
                type_,
            } => {
                write!(f, "{}: {} = {} {} {};", dest, type_, op, left, right)
                // match op {
                //     MathOp::ComputeOp(compute_op) =>  write!(f, "{}: {} = {} {} {};", dest, type_, compute_op, left, right),
                //     MathOp::CondOp(cond_op) =>  write!(f, "{}: {} = {} {} {};", dest, type_, left, cond_op, right),
                // }
            }
            AbstractInstruction::Assign { src, dest, type_ } => match type_ {
                Some(type_) => write!(f, "{}: {} =  {};", dest, type_, src),
                None => write!(f, "{} = {};", dest, src),
            },

            AbstractInstruction::Ret { src } => write!(f, "ret {}", src),

            AbstractInstruction::Value {
                args,
                dest,
                funcs,
                labels,
                src,
                type_,
            } => {
                match type_ {
                    Some(type__) => write!(f, "{dest}: {type__} = {src}")?,
                    None => write!(f, "{dest} = {src}")?,
                }
                for func in funcs {
                    write!(f, " @{func}")?;
                }
                for arg in args {
                    write!(f, " {arg}")?;
                }
                for label in labels {
                    write!(f, " .{label}")?;
                }
                write!(f, ";")
            }

            AbstractInstruction::Effect {
                op,
                args,
                funcs,
                labels,
            } => {
                write!(f, "{op}")?;
                for func in funcs {
                    write!(f, " @{func}")?;
                }
                for arg in args {
                    write!(f, " {arg}")?;
                }
                for label in labels {
                    write!(f, " .{label}")?;
                }
                write!(f, ";")
            }
        }
    }
}

// pub type AbstractType = String;
#[derive(Debug, Clone)]
pub enum AbstractType {
    /// For example `bool` => `Primitive("bool")`
    Type(String),
    /// For example `ptr<bool>` => `Parameterized("ptr", Box::new(Primitive("bool")))`
    Pointer(String, Box<Self>),
}

impl Display for AbstractType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AbstractType::Type(primitive) => {
                write!(f, "{}", primitive)
            }
            AbstractType::Pointer(_, _, ..) => Ok(()),
        }
    }
}

// #[derive(Debug, Clone)]
// pub enum ConstOps {
//     Const,
// }

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i32),
    // Bool(bool),
    // Float(f64),
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}", self.to_string())
        match self {
            Literal::Int(i) => write!(f, "{}", i),
        }
    }
}
