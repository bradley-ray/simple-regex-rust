enum State {
    Stay,
    Fail,
    Done,
}

#[derive(Debug, PartialEq)]
pub enum Op {
    NoOp,
    Cmp(char),
    AtLeast(u64),
    AtMost(u64),
    Final,
}

impl Op {
    fn run(&self, chr: Option<char>, num: Option<u64>) -> bool {
        match self {
            Op::Cmp(op) => *op == chr.unwrap(),
            Op::AtLeast(n) => *n <= num.unwrap(),
            _ => true,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RegExpr{ ops: Vec<Op> }


fn is_quantifier(token: char) -> bool {
    token == '*' || token == '+' || token == '?'
}

pub fn compile(src: &str) -> Result<RegExpr, &str> {
    let mut ops = Vec::with_capacity(64);
    let mut valid_quant = false;
    for (i, token) in src.chars().enumerate() {
        if is_quantifier(token) && valid_quant {
            valid_quant = false;
            ops.insert(i-1, match token {
                '*' => Op::AtLeast(0),
                '+' => Op::AtLeast(1),
                _   => Op::AtMost(1),
            });
        } else if !is_quantifier(token) {
            valid_quant = true;
            ops.push(match token {
                '.' => Op::NoOp,
                chr => Op::Cmp(chr),
            })
        } else {
            return Err("invalid quantifier position");
        }
    }

    ops.push(Op::Final);
    Ok(RegExpr{ ops })
}

// repeat operation n times
fn repeat_op(ops: &[Op], idx: usize, src: &str) -> (bool, bool, u64) {
    let mut at_most = false;
    let op = ops.get(idx).unwrap();
    let num = match op {
        &Op::AtLeast(n) => n,
        &Op::AtMost(n) => {at_most=true; n},
        _ => panic!("invalid repeat op"),
    };
    let mut count = 0;
    let next_op = ops.get(idx+1).unwrap();
    for token in src.chars() {
        let success = next_op.run(Some(token), None);
        if !success || (at_most && count == num) {
            break;
        }
        count += 1;
    }
    let successful = op.run(None, Some(count));

    (successful, count == 0 && successful, count)
}

// Finite state machine over source string
fn iter_src(ops: &[Op], idx: usize, src: &str) -> (State, usize, usize) {
    let mut op_idx = idx;
    let mut len = 0;
    for (i, token) in src.chars().enumerate() {
        let successful;
        let op = ops.get(op_idx).unwrap();
        match op {
            Op::Final => return (State::Done, len, op_idx),
            Op::AtLeast(_) | Op::AtMost(_) => {
                let src_slice = &src[i..];
                let result = repeat_op(ops, op_idx, src_slice);
                op_idx += 1;
                successful = result.0;
                len += result.2 as usize;
                // return to this character on next iteration
                if result.1 {
                    return (State::Stay, i, op_idx+1);
                }
            },
            _ => { 
                len += 1; 
                successful = op.run(Some(token), None);
            },
        };
        if !successful {
            break
        }
        op_idx += 1;
        len += 1;
    }

    (State::Fail, 0, 0)
}

impl RegExpr {
    // Execute the regular expression on source string
    fn run(&self, src: &str) -> Option<(usize, usize)> {
        let mut chr_idx = 0; // global run character idx
        let mut src_idx = 0; // local run character idx
        let mut op_idx = 0;
        loop {
            let tokens = &src[src_idx..];
            let result = iter_src(&self.ops, op_idx, tokens);
            match result.0 {
                State::Stay => {
                    src_idx += result.1;
                    op_idx = result.2
                },
                State::Fail => {
                    chr_idx += 1;
                    src_idx = chr_idx;
                    op_idx = 0;
                },
                State::Done => {
                    return Some((chr_idx,result.1));
                },
            }
            if src_idx >= src.len() {
                return None;
            }
        }
    }

    pub fn contain_match(&self,  src: &str) ->  bool {
        match self.run(src) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn replace(&self, src: &str, tgt: &str) -> Option<String> {
        let (idx, len) = match self.run(src) {
            Some(result) => (result.0, result.1),
            None => return None,
        };

        let new_str = String::from(&src[0..idx]) + tgt + &src[idx+len..];
        Some(new_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_valid_test() {
        let re = "abc.*[]+";
        let tgt = vec![Op::Cmp('a'), Op::Cmp('b'), Op::Cmp('c'), 
                       Op::AtLeast(0), Op::NoOp, Op::Cmp('['), 
                       Op::AtLeast(1), Op::Cmp(']'), Op::Final];
        let res = compile(re).unwrap();
        assert_eq!(tgt, res.ops);
    }

    #[test]
    fn compile_invalid_test() {
        let re = "*abc.[]+";
        assert_eq!(Err("invalid quantifier position"), compile(re));
        let re = "abc.[]++";
        assert_eq!(Err("invalid quantifier position"), compile(re));
        let re = "abc.+[]+?";
        assert_eq!(Err("invalid quantifier position"), compile(re));
    }

    #[test]
    fn contains_match_valid_test() {
        let re = "abce*k[]+";
        let src = "ab[]keftabck[]]]";
        let regex = compile(re).unwrap();
        let is_match = regex.contain_match(src);
        assert!(is_match);
    }

    #[test]
    fn contains_match_invalid_test() {
        let re = "ab*a[]+";
        let regex = compile(re).unwrap();
        let is_match = regex.contain_match("abc[]]]]");
        assert!(!is_match);
    }

    #[test]
    fn replace_valid_test() {
        let re = "abce*a[]+";
        let src = "ab[]keftabca[]]]asfasdf";
        let tgt = "hello";
        let regex = compile(re).unwrap();
        let result = regex.replace(src, tgt);
        assert_eq!(Some(String::from("ab[]kefthelloasfasdf")), result);
    }

    #[test]
    fn replace_invalid_test() {
        let re = "abce*a[]+";
        let src = "eftabc[]]]asfasdf";
        let tgt = "hello";
        let regex = compile(re).unwrap();
        let result = regex.replace(src, tgt);
        assert_eq!(None, result);
    }
}
