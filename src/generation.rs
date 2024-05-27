use crate::parser::*;
use std::collections::HashMap;
pub struct CodeGeneration {
    nodes: Vec<Box<dyn Node>>,
    pub variables: HashMap<String, usize>,
    stack_size: usize,
    valid_exit: bool,
}
impl CodeGeneration {
    pub fn new(nodes: Vec<Box<dyn Node>>) -> Self {
        Self {
            nodes,
            variables: HashMap::new(),
            stack_size: 0,
            valid_exit: false,
        }
    }

    pub fn generate_asm(&mut self) -> Result<String, String> {
        let mut generated_assembly = String::from("global _start\n_start:\n");
        for node in &self.nodes {
            match node.nodetype() {
                NodeType::Exit => {
                    self.valid_exit = true;
                    let node_exit_type_param1 =
                        &node.as_any().downcast_ref::<NodeExit>().unwrap().expr;
                    match node_exit_type_param1 {
                        ExpressionTypes::Int(int_node) => {
                            let exit_code = int_node.int_literal.value.as_ref().unwrap();
                            generated_assembly += "    mov rax, 60\n";
                            generated_assembly += format!("    mov rdi, {exit_code}\n").as_str();
                            generated_assembly += "    syscall\n";
                        }
                        ExpressionTypes::Identifier(ident_node) => {
                            let identifier_name = ident_node.ident.value.as_ref().unwrap();
                            let stack_size_of_var = self.variables.get(identifier_name);
                            if let Some(var_stack) = stack_size_of_var {
                                generated_assembly += format!(
                                    "    push QWORD [rsp + {}]\n",
                                    (self.stack_size - var_stack - 1) * 8
                                )
                                .as_str();
                                generated_assembly += "    mov rax, 60\n";
                                generated_assembly += "    pop rdi\n";
                                generated_assembly += "    syscall\n"
                            } else {
                                return Err(format!(
                                    "ERROR: {identifier_name} identifier does not exist"
                                ));
                            }
                        }
                    }

                    // .int_literal
                }
                NodeType::Variable => {
                    let var_node = node.as_any().downcast_ref::<NodeVariable>().unwrap();
                    let identifier = var_node.identifier.value.as_ref().unwrap();

                    if self.variables.contains_key(identifier) {
                        return Err(format!("identifier {identifier} is already defined"));
                    }
                    match var_node.expr.nodetype() {
                        NodeType::ExpressionInt => {
                            let expr = var_node
                                .expr
                                .as_any()
                                .downcast_ref::<NodeExpressionInt>()
                                .unwrap();
                            self.variables
                                .insert(identifier.to_string(), self.stack_size);
                            generated_assembly += format!(
                                "    mov rax, {}\n",
                                expr.int_literal.value.as_ref().unwrap()
                            )
                            .as_str();
                            generated_assembly += "    push rax\n";
                            self.stack_size += 1; // increasing the stack size (+1)
                        }
                        NodeType::ExpressionIdent => {
                            let expr = var_node
                                .expr
                                .as_any()
                                .downcast_ref::<NodeExpressionIdentifier>()
                                .unwrap()
                                .ident
                                .value
                                .as_ref()
                                .unwrap();
                            let varident = var_node.identifier.value.as_ref().unwrap();
                            if let Some(pointed_assignment) = self.variables.get(expr) {
                                generated_assembly += format!(
                                    "    PUSH QWORD [rsp + {}]\n",
                                    (self.stack_size - pointed_assignment - 1) * 8
                                )
                                .as_str();
                                generated_assembly += "    pop rax\n";
                                generated_assembly +=
                                    format!("    push rax; {} = {}\n", varident, expr).as_str();
                                self.variables.insert(varident.to_owned(), self.stack_size);
                                self.stack_size += 1;
                            } else {
                                return Err(format!("Unknown variable {expr}"));
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        if self.valid_exit == false {
            return Err("Program is missing a valid EXIT CODE".to_string());
        }

        Ok(generated_assembly)
    }
}
