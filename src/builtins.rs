use super::*;
use indexmap::IndexMap;

mod arithmetic;

pub type Builtin = fn(&mut Vec<Number>) -> Result<'static, ()>;

pub fn generate_builtins() -> IndexMap<&'static str, Builtin> {
    let mut i = IndexMap::new();

    i.insert("+", arithmetic::add as Builtin);
    i.insert("-", arithmetic::sub);
    i.insert("*", arithmetic::mul);
    i.insert("/", arithmetic::div);
    i.insert("log", arithmetic::log);
    i.insert("**", arithmetic::powf);
    i.insert("%", arithmetic::rem);

    i.insert("log2", arithmetic::log2);
    i.insert("log10", arithmetic::log10);
    i.insert("sqrt", arithmetic::sqrt);
    i.insert("deg", arithmetic::deg);
    i.insert("rad", arithmetic::rad);

    i
}
