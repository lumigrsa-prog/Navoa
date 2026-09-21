use std::collections::HashMap;
use navoa_bytecode::Chunk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capabilities {
    pub allow_io: bool,
    pub allow_env: bool,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            allow_io: false,
            allow_env: false,
        }
    }
}

pub struct Vvm {
    stack: Vec<f64>,
    memory: HashMap<String, f64>,
    capabilities: Capabilities,
}

impl Vvm {
    pub fn new(capabilities: Capabilities) -> Self {
        Self {
            stack: Vec::new(),
            memory: HashMap::new(),
            capabilities,
        }
    }

    pub fn run_chunk(&mut self, chunk: &Chunk) -> Result<(), String> {
        let mut ip = 0; // Instruction Pointer

        while ip < chunk.code.len() {
            let opcode = chunk.code[ip];
            ip += 1;

            match opcode {
                0x01 => { // PushNum
                    let const_idx = chunk.code[ip] as usize;
                    ip += 1;
                    let val = chunk.constants.get(const_idx)
                        .ok_or_else(|| format!("Constante inválida no índice {}", const_idx))?;
                    self.stack.push(*val);
                }
                0x02 => { // StoreVar
                    let str_idx = chunk.code[ip] as usize;
                    ip += 1;
                    let var_name = chunk.strings.get(str_idx)
                        .ok_or_else(|| format!("String inválida no índice {}", str_idx))?;
                    let val = self.stack.pop()
                        .ok_or_else(|| "Stack underflow ao guardar variável".to_string())?;
                    self.memory.insert(var_name.clone(), val);
                }
                0x03 => { // LoadVar
                    let str_idx = chunk.code[ip] as usize;
                    ip += 1;
                    let var_name = chunk.strings.get(str_idx)
                        .ok_or_else(|| format!("String inválida no índice {}", str_idx))?;
                    let val = self.memory.get(var_name)
                        .ok_or_else(|| format!("Variável '{}' não encontrada", var_name))?;
                    self.stack.push(*val);
                }
                0x04 => { // Add
                    let b = self.stack.pop().ok_or("Stack underflow em Add")?;
                    let a = self.stack.pop().ok_or("Stack underflow em Add")?;
                    self.stack.push(a + b);
                }
                0x05 => { // Sub
                    let b = self.stack.pop().ok_or("Stack underflow em Sub")?;
                    let a = self.stack.pop().ok_or("Stack underflow em Sub")?;
                    self.stack.push(a - b);
                }
                0x06 => { // Mul
                    let b = self.stack.pop().ok_or("Stack underflow em Mul")?;
                    let a = self.stack.pop().ok_or("Stack underflow em Mul")?;
                    self.stack.push(a * b);
                }
                0x07 => { // Div
                    let b = self.stack.pop().ok_or("Stack underflow em Div")?;
                    let a = self.stack.pop().ok_or("Stack underflow em Div")?;
                    if b == 0.0 {
                        return Err("Divisão por zero no Bytecode!".to_string());
                    }
                    self.stack.push(a / b);
                }
                0x08 => { // Print
                    if !self.capabilities.allow_io {
                        return Err(" [Segurança] Operação de I/O negada (Deny-by-Default)".to_string());
                    }
                    let val = self.stack.pop().ok_or("Stack underflow em Print")?;
                    println!("[VVM Bytecode Output]: {}", val);
                }
                0x00 => { // Halt
                    break;
                }
                _ => return Err(format!("Opcode desconhecido: 0x{:02X}", opcode)),
            }
        }

        Ok(())
    }

    pub fn get_var(&self, id: &str) -> Option<&f64> {
        self.memory.get(id)
    }
}
