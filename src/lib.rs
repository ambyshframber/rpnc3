use indexmap::IndexMap;
use lazy_static::lazy_static;
use builtins::*;

mod builtins;

type Number = f64;

lazy_static! {
    static ref BUILTINS: IndexMap<&'static str, Builtin> = generate_builtins();
}

pub struct Shell {
    pub stack: Vec<Number>,

    defs_dict: IndexMap<String, Box<[u32]>>
}
impl Shell {
    pub fn new() -> Self {
        Self { stack: Vec::new(), defs_dict: IndexMap::new() }
    }

    pub fn exec_line<'a>(&mut self, line: &'a str) -> Result<'a, String> {
        let mut stack_backup = self.stack.clone();

        let mut sret = String::new();

        for word in line.split_ascii_whitespace() {
            match self.exec_word(word) {
                Ok(None) => {},
                Ok(Some(s)) => {
                    sret.push_str(&s);
                    sret.push('\n')
                }

                Err(e) => { // swap stack back into place, propagate error
                    std::mem::swap(&mut self.stack, &mut stack_backup);
                    return Err(e)
                }
            }
        }

        Ok(sret)
    }
    
    pub fn exec_word<'a>(&mut self, word: &'a str) -> Result<'a, Option<String>> {
        match word {
            "." => self.stack.pop().ok_or(Underflow).map(|v| v.to_string()).map(Some),
    
            _ => {
                if let Some(def) = self.defs_dict.get(word) {
                    todo!()
                }
                else if let Some(b) = BUILTINS.get(word) {
                    b(&mut self.stack)?;
                }
                else {
                    return Err(Undefined(word))
                }
                Ok(None)
            }
        }
    }
}

pub type Result<'a, T> = std::result::Result<T, Error<'a>>;
use Error::*;
pub enum Error<'a> {
    Underflow,
    Div0,
    Undefined(&'a str),
    IpEscape,
    UndefinedCompiled // exclusively internal error
}
