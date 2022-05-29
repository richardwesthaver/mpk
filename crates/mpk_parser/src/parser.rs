//! MPK_PARSER -- PARSER
//!
//! Parse tokens into an unvalidated Program.
use mpk_hash::FxHashMap as HashMap;
use pest::Parser;

use crate::ast::*;
use crate::err::{Error, ErrorVariant, PestError};

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MpkParser;

pub fn parse(source: &str) -> Result<Program, Error> {
  let mut ast = vec![];
  let pairs = MpkParser::parse(Rule::program, source)?;

  for pair in pairs {
    match pair.as_rule() {
      Rule::expr => {
        ast.push(build_ast_from_expr(pair)?);
      }
      _ => {}
    }
  }

  Ok(ast)
}

fn build_ast_from_expr(pair: pest::iterators::Pair<Rule>) -> Result<AstNode, Error> {
  match pair.as_rule() {
    Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
    Rule::dyadic => {
      let mut pair = pair.into_inner();
      let lhspair = pair.next().unwrap();
      let lhs = build_ast_from_expr(lhspair)?;
      let verb = pair.next().unwrap();
      let av_or_rhs = pair.next().unwrap();
      // check if this is an adverb
      let adverb = match parse_ad_verb(av_or_rhs.clone()) {
        Ok(av) => av,
        Err(_) => None,
      };
      let rhspair = if adverb.is_some() {
        pair.next().unwrap()
      } else {
        av_or_rhs
      };

      let rhs = build_ast_from_expr(rhspair)?;
      parse_dyadic_verb(verb, adverb, lhs, rhs)
    }
    Rule::monadic => {
      let mut pair = pair.into_inner();
      let verb = pair.next().unwrap();
      let av_or_rhs = pair.next().unwrap();
      // check if this is an adverb
      let adverb = match parse_ad_verb(av_or_rhs.clone()) {
        Ok(av) => av,
        Err(_) => None,
      };
      let expr = if adverb.is_some() {
        pair.next().unwrap()
      } else {
        av_or_rhs
      };
      let expr = build_ast_from_expr(expr)?;
      parse_monadic_verb(verb, adverb, expr)
    }
    Rule::nouns => {
      let nouns: Program = pair
        .into_inner()
        .map(|p| build_ast_from_noun(p).unwrap())
        .collect();
      // If there's just a single noun, return it without
      // wrapping it in a List.
      match nouns.len() {
        1 => Ok(nouns.get(0).unwrap().clone()),
        _ => Ok(AstNode::List(nouns)),
      }
    }
    Rule::assgmt => {
      let mut pair = pair.into_inner();
      let n = pair.next().unwrap();
      let name = parse_name(n)?;
      // skip ASSIGN token
      pair.next().unwrap();
      let val = pair.next().unwrap();
      let expr = build_ast_from_expr(val)?;
      Ok(AstNode::Var {
        name,
        expr: Box::new(expr),
      })
    }
    Rule::sysExpr => {
      let mut pair = pair.into_inner();
      let verb = pair.next().unwrap();
      let args = if let Some(e) = pair.next() {
        Some(build_ast_from_expr(e)?)
      } else {
        None
      };
      parse_sys_verb(verb, args)
    }
    Rule::fnExpr => {
      let mut pair = pair.into_inner();
      let mut args = vec![];
      let lb_or_expr = pair.next().unwrap();
      if lb_or_expr.as_str() == "[" {
        while let Some(i) = pair.next() {
          if i.as_str() == "]" {
            break;
          } else {
            match parse_fn_arg(i) {
              Ok(a) => args.push(a),
              Err(e) => eprintln!("{}", e),
            }
          }
        }
        parse_fn_expr(Some(args), build_ast_from_expr(pair.next().unwrap())?)
      } else {
        parse_fn_expr(None, build_ast_from_expr(lb_or_expr)?)
      }
    }
    // TODO read up on different forms of syntax for fn calls -- `x[1;2;3];x 1; (x 1 2); x (1;2;3)`
    Rule::fnCall => {
      let mut pair = pair.into_inner();
      // can be parsed with `parse_fn_arg` since it's just a name and we need a String.
      let name = parse_fn_arg(pair.next().unwrap())?;
      let mut args = vec![];
      while let Some(a) = pair.next() {
        match build_ast_from_expr(a) {
          Ok(a) => args.push(a),
          Err(e) => eprintln!("{}", e),
        }
      }
      if !args.is_empty() {
        parse_fn_call(name, Some(args))
      } else {
        parse_fn_call(name, None)
      }
    }
    _ => Err(Error::PestErr(PestError::new_from_span(
      ErrorVariant::CustomError {
        message: "unexpected expression".to_string(),
      },
      pair.as_span(),
    ))),
  }
}

fn parse_dyadic_verb(
  pair: pest::iterators::Pair<Rule>,
  adverb: Option<AdVerb>,
  lhs: AstNode,
  rhs: AstNode,
) -> Result<AstNode, Error> {
  let verb = match pair.as_str() {
    "+" => Ok(DyadicVerb::Plus),
    "-" => Ok(DyadicVerb::Minus),
    "*" => Ok(DyadicVerb::Times),
    "%" => Ok(DyadicVerb::Divide),
    "!" => Ok(DyadicVerb::Mod),
    "&" => Ok(DyadicVerb::Min),
    "|" => Ok(DyadicVerb::Max),
    "<" => Ok(DyadicVerb::Less),
    ">" => Ok(DyadicVerb::More),
    "=" => Ok(DyadicVerb::Equal),
    "~" => Ok(DyadicVerb::Match),
    "," => Ok(DyadicVerb::Concat),
    "^" => Ok(DyadicVerb::Except),
    "#" => Ok(DyadicVerb::Take),
    "_" => Ok(DyadicVerb::Drop),
    "$" => Ok(DyadicVerb::Cast),
    "?" => Ok(DyadicVerb::Find),
    "@" => Ok(DyadicVerb::At),
    "." => Ok(DyadicVerb::Dot),
    e => Err(format!("invalid dyadic verb: {}", e.to_string())),
  };
  if let Ok(verb) = verb {
    Ok(AstNode::Dyad {
      lhs: Box::new(lhs),
      verb,
      adverb,
      rhs: Box::new(rhs),
    })
  } else {
    Err(Error::PestErr(PestError::new_from_span(
      ErrorVariant::CustomError {
        message: "invalid dyadic verb".to_string(),
      },
      pair.as_span(),
    )))
  }
}

fn parse_monadic_verb(
  pair: pest::iterators::Pair<Rule>,
  adverb: Option<AdVerb>,
  expr: AstNode,
) -> Result<AstNode, Error> {
  let verb = match pair.as_str() {
    "+" => Ok(MonadicVerb::Flip),
    "-" => Ok(MonadicVerb::Negate),
    "*" => Ok(MonadicVerb::First),
    "%" => Ok(MonadicVerb::Sqrt),
    "<" => Ok(MonadicVerb::Asc),
    ">" => Ok(MonadicVerb::Desc),
    "=" => Ok(MonadicVerb::Group),
    "~" => Ok(MonadicVerb::Not),
    "," => Ok(MonadicVerb::Enlist),
    "#" => Ok(MonadicVerb::Count),
    "_" => Ok(MonadicVerb::Floor),
    "$" => Ok(MonadicVerb::String),
    "?" => Ok(MonadicVerb::Distinct),
    "@" => Ok(MonadicVerb::Type),
    "." => Ok(MonadicVerb::Eval),
    e => Err(format!("invalid monadic verb: {}", e.to_string())),
  };
  if let Ok(verb) = verb {
    Ok(AstNode::Monad {
      verb,
      adverb,
      expr: Box::new(expr),
    })
  } else {
    Err(Error::PestErr(PestError::new_from_span(
      ErrorVariant::CustomError {
        message: "invalid monadic verb".to_string(),
      },
      pair.as_span(),
    )))
  }
}

fn parse_ad_verb(pair: pest::iterators::Pair<Rule>) -> Result<Option<AdVerb>, Error> {
  match pair.as_str() {
    "'" => Ok(Some(AdVerb::Each)),
    "/" => Ok(Some(AdVerb::Over)),
    "\\" => Ok(Some(AdVerb::Scan)),
    "':" => Ok(Some(AdVerb::EachPrior)),
    "\\:" => Ok(Some(AdVerb::EachLeft)),
    "/:" => Ok(Some(AdVerb::EachRight)),
    _ => Err(Error::PestErr(PestError::new_from_span(
      ErrorVariant::CustomError {
        message: "invalid adverb".to_string(),
      },
      pair.as_span(),
    ))),
  }
}

fn parse_sys_verb(
  pair: pest::iterators::Pair<Rule>,
  args: Option<AstNode>,
) -> Result<AstNode, Error> {
  let verb = match pair.as_str().trim().strip_prefix("\\").unwrap() {
    "\\" => Ok(SysVerb::Exit),
    "v" => Ok(SysVerb::Vars),
    "w" => Ok(SysVerb::Work),
    "l" => Ok(SysVerb::Import),
    "t" => Ok(SysVerb::Timeit),
    "sesh" => Ok(SysVerb::Sesh),
    "proxy" => Ok(SysVerb::Proxy),
    "db" => Ok(SysVerb::Db),
    e => Err(format!("invalid sys verb: {}", e.to_string())),
  };
  if verb.is_ok() {
    Ok(AstNode::SysFn {
      verb: verb.unwrap(),
      args: args.map(|e| Box::new(e)),
    })
  } else {
    Err(Error::PestErr(PestError::new_from_span(
      ErrorVariant::CustomError {
        message: "invalid sysfn".to_string(),
      },
      pair.as_span(),
    )))
  }
}

fn parse_fn_arg(pair: pest::iterators::Pair<Rule>) -> Result<Name, Error> {
  match build_ast_from_noun(pair) {
    Ok(AstNode::Atom(n)) => {
      if let Atom::Name(n) = n {
        Ok(n)
      } else {
        Err(Error::InvalidNoun("<name>".to_string(), n.to_string()))
      }
    }
    Ok(e) => {
      let e = e.to_string();
      Err(Error::InvalidNoun("<name>".to_string(), e.to_string()))
    }
    Err(e) => Err(e),
  }
}

fn parse_fn_expr(args: Option<Vec<Name>>, expr: AstNode) -> Result<AstNode, Error> {
  Ok(AstNode::UserFn {
    args,
    expr: Box::new(expr),
  })
}

fn parse_fn_call_args(
  pair: pest::iterators::Pair<Rule>,
) -> Result<Option<Vec<AstNode>>, Error> {
  let args = None;
  Ok(args)
}

fn parse_fn_call(name: Name, args: Option<Vec<AstNode>>) -> Result<AstNode, Error> {
  Ok(AstNode::FnCall { name, args })
}

fn parse_name(pair: pest::iterators::Pair<Rule>) -> Result<Name, Error> {
  Ok(Name::from(pair.as_str()))
}

fn parse_int(pair: pest::iterators::Pair<Rule>) -> Result<Integer, Error> {
  let i = pair.as_str();
  let (sign, int) = match &i[..1] {
    "-" => (-1, &i[1..]),
    _ => (1, &i[..]),
  };
  match sign {
    1 => {
      if let Ok(i) = int.parse::<u8>() {
        Ok(Integer::G(i))
      } else if let Ok(i) = int.parse::<u16>() {
        Ok(Integer::H(i))
      } else if let Ok(i) = int.parse::<u32>() {
        Ok(Integer::I(i))
      } else if let Ok(i) = int.parse::<i64>() {
        Ok(Integer::J(i))
      } else {
        Err(Error::Num(i.to_string()))
      }
    }
    -1 => {
      if let Ok(i) = int.parse::<i64>() {
        Ok(Integer::J(sign * i))
      } else {
        Err(Error::Num(i.to_string()))
      }
    }
    _ => Err(Error::Num(i.to_string())),
  }
}

fn parse_float(pair: pest::iterators::Pair<Rule>) -> Result<Float, Error> {
  let f = pair.as_str();
  let (sign, flt) = match &f[..1] {
    "-" => (-1., &f[1..]),
    _ => (1., &f[..]),
  };
  if let Ok(mut f) = flt.parse::<f32>() {
    if f != 0.0 {
      f *= sign;
    }
    Ok(Float::E(f))
  } else if let Ok(mut f) = flt.parse::<f64>() {
    if f != 0.0 {
      f *= sign as f64;
    }
    Ok(Float::F(f))
  } else {
    Err(Error::Num(f.to_string()))
  }
}

fn build_ast_from_noun(pair: pest::iterators::Pair<Rule>) -> Result<AstNode, Error> {
  match pair.as_rule() {
    Rule::int => Ok(AstNode::Atom(Atom::Int(parse_int(pair)?))),
    Rule::decimal => Ok(AstNode::Atom(Atom::Float(parse_float(pair)?))),
    Rule::string => {
      let str = pair.as_str();
      // Strip leading and ending quotes.
      let str = &str[1..str.len() - 1];
      // Escaped string quotes become single quotes here.
      let str = str.replace("\"\"", "\"");
      let str: Vec<Char> = str
        .into_bytes()
        .into_iter()
        .map(|c| Char::from(c))
        .collect();
      Ok(AstNode::Str(str))
    }
    Rule::symbol => {
      let sym = pair.as_str().strip_prefix("`").unwrap();
      Ok(AstNode::Symbol(Name::from(sym)))
    }
    Rule::list => {
      let mut lst = vec![];
      for i in pair.into_inner() {
        match build_ast_from_expr(i) {
          Ok(p) => lst.push(p),
          Err(_) => (),
        }
      }
      Ok(AstNode::List(lst))
    }
    Rule::dict => {
      let mut dict = HashMap::default();

      let mut pair = pair.into_inner();

      while pair.peek().is_some() {
        let key = match parse_name(pair.next().unwrap()) {
          Ok(k) => Some(k),
          _ => None,
        };

        match pair.next() {
          Some(s) => {
            if s.as_str() == ":" {
              let val = pair.next().unwrap();
              let val = match build_ast_from_expr(val) {
                Ok(v) => Some(v),
                _ => None,
              };
              if let Some(k) = key {
                if let Some(v) = val {
                  dict.insert(k, v);
                } else {
                  dict.insert(k, AstNode::List(vec![]));
                }
              };
            };
          }
          None => {
            if let Some(k) = key {
              dict.insert(k, AstNode::List(vec![]));
            };
          }
        }
      }
      Ok(AstNode::Dict(dict))
    }
    Rule::table => {
      let mut tbl = HashMap::default();
      let mut pair = pair.into_inner();
      while pair.peek().is_some() {
        let col = match parse_name(pair.next().unwrap()) {
          Ok(k) => Some(k),
          _ => None,
        };
        match pair.next() {
          Some(s) => {
            if s.as_str() == ":" {
              let val = pair.next().unwrap();
              let val = match build_ast_from_expr(val) {
                Ok(v) => Some(v),
                _ => None,
              };
              if let Some(k) = col {
                if let Some(v) = val {
                  tbl.insert(k, v);
                } else {
                  tbl.insert(k, AstNode::List(vec![]));
                }
              };
            };
          }
          None => {
            if let Some(k) = col {
              tbl.insert(k, AstNode::List(vec![]));
            };
          }
        }
      }
      Ok(AstNode::Table(tbl))
    }
    Rule::expr => build_ast_from_expr(pair),
    Rule::name => Ok(AstNode::Atom(Atom::Name(parse_name(pair)?))),
    _ => Err(Error::PestErr(PestError::new_from_span(
      ErrorVariant::CustomError {
        message: "invalid noun".to_string(),
      },
      pair.as_span(),
    ))),
  }
}

#[cfg(test)]
pub mod tests {
  use super::*;
  use crate::ast::*;
  use crate::err::{Error, ErrorVariant, PestError};
  #[test]
  fn parse_program() {
    assert!(parse("f:{x+y+z};x:[[]c:1 2 3];2+2+/3 4 5;").is_ok())
  }
  #[test]
  fn parse_float() {
    assert_eq!(
      parse("1.0 ; ;").unwrap(),
      vec![AstNode::Atom(Atom::Float(Float::E(1.0)))]
    );
    assert_eq!(
      parse("1000101010101.0 ; ;").unwrap(),
      vec![AstNode::Atom(Atom::Float(Float::F(10000000.012345)))]
    );

    assert_eq!(
      parse("-1.0 ; ;").unwrap(),
      vec![AstNode::Monad {
        verb: MonadicVerb::Negate,
        adverb: None,
        expr: Box::new(AstNode::Atom(Atom::Float(Float::E(1.0))))
      }]
    );
  }
}
