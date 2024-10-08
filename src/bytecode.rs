
#[derive(Default, Clone, Copy, Debug)]
pub struct LineNo {
    pub value: usize
}

impl LineNo {
    fn new(value:usize) -> Self {
        LineNo {
            value
        }
    }
}
#[derive(Debug, Clone)]
pub enum Op {
    Return, 
    // Constant stored at a particular index or idx
    Constant(usize),
    Nil, 
    True,
    False,
    Negate,
    Add,
    Subtract,
    Multiply,
    Divide,
    Not,
    Equal,
    Greater,
    Less,
    Print,
    Pop,
}

#[derive(Default, Clone, Debug)]
pub struct Function {
    pub arity: u8, 
    pub chunk: Chunk,
    pub name : String,
}


// Constant structs stores the value
#[derive(Debug, Clone)]
pub enum Constant {
    Number(f64),
    String(String)
}

impl std::fmt::Display for Constant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Constant::Number(num) => write!(f, "{}", num),
            Constant::String(string)    => write!(f, "\"{}\"", string),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Chunk {
    // Code is just Opcode and LineNo
    pub code: Vec<(Op, LineNo)>,
    pub constants: Vec<Constant>,
}

impl Chunk {
    pub fn add_constant(&mut self, val:Constant) -> usize {
        let new_idx = self.constants.len();
        self.constants.push(val);
        return new_idx
    }

    /// To add a `String`, we must know if it is already there.
    /// This methods does that exactly. 
    fn find_string(&self,to_find: &str ) -> Option<usize> {
        self.constants.iter().position(|c|{
            if let Constant::String(value) = c {
                value == to_find
            } 
            else {
                false
            }
        })
    }

    pub fn add_constant_string(&mut self, to_add: &str) -> usize {
        if let Some(id)  = self.find_string(to_add) {
            id
        }
        else {
            self.add_constant(Constant::String(to_add.to_string()))
        }
    } 

    pub fn add_constant_number(&mut self, num: f64) -> usize {
        if let Some(id) = self.find_number(num) {
            id
        }
        else {
            self.add_constant(Constant::Number(num))
        }
    }
    
    /// Checks if the number is already in the `constants` field. 
    fn find_number(&self, to_find: f64) -> Option<usize> {
        self.constants.iter().position(|num| {
            if let Constant::Number(value) = num {
                // If the difference between the two numbers is so low
                // that it is the lowest value that can be represented by
                // our systems then they are almost the same.
                // This prevents us to unnecessarily compare the float numbers

                (to_find - *value).abs() < f64::EPSILON
            }
            else {
                false
            }
        })
    }
}