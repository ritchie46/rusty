use crate::code::{Instructions, OpCode, Operand};
use monkey::eval::object::Object;
use monkey::parser::ast::{Expression, Statement};
use std::str::Bytes;

#[derive(Debug, Clone)]
pub struct Bytecode<'compiler> {
    pub instructions: &'compiler Instructions,
    pub constants: &'compiler Vec<Object>,
}

#[derive(Debug)]
struct EmittedInstruction {
    pub oc: OpCode,
    pub position: usize,
}

pub struct Compiler {
    instructions: Instructions,
    constants: Vec<Object>,
    last_instruction: Option<EmittedInstruction>,
    before_last_instruction: Option<EmittedInstruction>,
}

impl Compiler {
    pub fn new() -> Compiler {
        Compiler {
            instructions: vec![],
            constants: vec![],
            last_instruction: None,
            before_last_instruction: None,
        }
    }

    pub fn bytecode(&self) -> Bytecode {
        Bytecode {
            instructions: &self.instructions,
            constants: &self.constants,
        }
    }

    pub fn compile_program(&mut self, program: &[Statement]) {
        for stmt in program {
            self.compile_stmt(stmt)
        }
    }

    fn compile_stmt(&mut self, stmt: &Statement) {
        match stmt {
            Statement::Expr(expr) => {
                self.compile_expr(expr);
                self.emit(OpCode::Pop, &[]);
            }
            Statement::Block(stmts) => {
                for stmt in stmts.iter() {
                    self.compile_stmt(stmt);
                }
            }
            _ => panic!(),
        }
    }

    fn compile_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Infix {
                left,
                operator,
                right,
            } => {
                // Reverse the constants to flip GT behavior to LT
                if operator == "<" {
                    self.compile_expr(right);
                    self.compile_expr(left);
                } else {
                    self.compile_expr(left);
                    self.compile_expr(right);
                }
                match &operator[..] {
                    "+" => {
                        self.emit(OpCode::Add, &[]);
                    }
                    "-" => {
                        self.emit(OpCode::Sub, &[]);
                    }
                    "*" => {
                        self.emit(OpCode::Mul, &[]);
                    }
                    "/" => {
                        self.emit(OpCode::Div, &[]);
                    }
                    ">" => {
                        self.emit(OpCode::GT, &[]);
                    }
                    "<" => {
                        self.emit(OpCode::GT, &[]);
                    }
                    "==" => {
                        self.emit(OpCode::Equal, &[]);
                    }
                    "!=" => {
                        self.emit(OpCode::NotEqual, &[]);
                    }
                    _ => panic!("Operand not known"),
                }
            }
            Expression::IntegerLiteral(v) => {
                let int = Object::Int(*v);
                let op = self.add_constant(int);
                self.emit(OpCode::Constant, &[op]);
            }
            Expression::Bool(v) => {
                if *v {
                    self.emit(OpCode::True, &[]);
                } else {
                    self.emit(OpCode::False, &[]);
                }
            }
            Expression::Prefix { operator, expr } => {
                self.compile_expr(expr);
                match &operator[..] {
                    "-" => {
                        self.emit(OpCode::Minus, &[]);
                    }
                    "!" => {
                        self.emit(OpCode::Bang, &[]);
                    }
                    _ => panic!(),
                }
            }
            Expression::IfExpression {
                condition,
                consequence,
                alternative,
            } => {
                self.compile_expr(condition);
                self.emit(OpCode::JumpNotTruthy, &[9999]);
                self.compile_stmt(consequence);

                if self.last_instruction_is_pop() {
                    self.remove_last_pop()
                }
            }
            _ => panic!(),
        };
    }

    /// returns memory location
    fn add_constant(&mut self, obj: Object) -> usize {
        self.constants.push(obj);
        self.constants.len() - 1
    }

    fn emit(&mut self, oc: OpCode, operands: &[Operand]) -> usize {
        let ins = oc.make(operands);
        let pos = self.add_instruction(&ins);

        // Keep track of previous instructions
        let last = EmittedInstruction { oc, position: pos };
        self.before_last_instruction = self.last_instruction.replace(last);
        pos
    }

    fn add_instruction(&mut self, instructions: &[u8]) -> usize {
        // position of start new instructions
        let pos = self.instructions.len();
        self.instructions.extend_from_slice(instructions);
        pos
    }

    fn last_instruction_is_pop(&self) -> bool {
        match &self.last_instruction {
            Some(emit_instr) => {
                if let OpCode::Pop = emit_instr.oc {
                    return true;
                }
            }
            _ => {}
        }

        false
    }

    fn remove_last_pop(&mut self) {
        let pos = match &self.last_instruction {
            Some(em_ins) => em_ins.position,
            _ => panic!(),
        };

        self.instructions.drain(pos..);

        let new_last = self.before_last_instruction.take();
        self.last_instruction.replace(new_last.unwrap());
    }
}
