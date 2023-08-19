use std::fmt::{Display, Formatter};

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
    pub instrs: Vec<AbstractCode>,
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
        for instr in &self.instrs {
            writeln!(f, "{}", instr)?;
        }
        write!(f, "}}")?;
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
    Value {
        args: Vec<String>,
        dest: String,
        funcs: Vec<String>,
        labels: Vec<String>,
        op: String,
        op_type: AbstractType,
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
            AbstractInstruction::Value { .. } => Ok(()),
            AbstractInstruction::Effect { .. } => Ok(()),
        }
    }
}

pub type AbstractType = String;

// impl Display for AbstractType {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         // write!(f,"")
//         match self {
//             AbstractType::Primitive(primitive) => { write!(f, "{}", primitive) }
//             AbstractType::Parameterized(_, _, ..) => Ok(())
//         }
//     }
// }

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
