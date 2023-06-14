use crate::*;
use crate::Error::*;

macro_rules! arith_2_oper {
    ($name:ident, $fn:path) => {
        pub fn $name(stack: &mut Vec<Number>) -> Result<'static, ()> {
            if stack.len() < 2 {
                return Err(Underflow)
            }
            let b = stack.pop().unwrap();
            let idx = stack.len() - 1;
            let a = stack[idx];
            let res = $fn(a, b);
            stack[idx] = res;
            Ok(())
        }
    };
}

use std::ops::*;

arith_2_oper!(add, Number::add);
arith_2_oper!(sub, Number::sub);
arith_2_oper!(mul, Number::mul);
arith_2_oper!(div, Number::div);
arith_2_oper!(log, Number::log);
arith_2_oper!(powf, Number::powf);
arith_2_oper!(rem, Number::rem);

macro_rules! arith_1_oper {
    ($name:ident, $fn:path) => {
        pub fn $name(stack: &mut Vec<Number>) -> Result<'static, ()> {
            if stack.len() < 1 {
                return Err(Underflow)
            }
            let idx = stack.len() - 1;
            let a = stack[idx];
            let res = $fn(a);
            stack[idx] = res;
            Ok(())
        }
    };
    ($name:ident, $fn:path, $v:expr) => {
        pub fn $name(stack: &mut Vec<Number>) -> Result<'static, ()> {
            if stack.len() < 1 {
                return Err(Underflow)
            }
            let idx = stack.len() - 1;
            let a = stack[idx];
            let res = $fn(a, $v);
            stack[idx] = res;
            Ok(())
        }
    };
}

arith_1_oper!(log2, Number::log2);
arith_1_oper!(log10, Number::log10);
arith_1_oper!(sqrt, Number::sqrt);
arith_1_oper!(rad, Number::to_radians);
arith_1_oper!(deg, Number::to_degrees);
