//! MPK_PARSER -- PARSER
//!
//! Parse tokens into an unvalidated Program.
use mpk_hash::FxHashMap as HashMap;
use pest::Parser;

use crate::ast::*;
use crate::err::{convert_error, Error, ErrorVariant, PestError};

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
      let name = pair.next().unwrap();
      // skip ASSIGN token
      pair.next().unwrap();
      let val = pair.next().unwrap();
      let expr = build_ast_from_expr(val)?;
      Ok(AstNode::Var {
        name: String::from(name.as_str()),
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

fn parse_fn_arg(pair: pest::iterators::Pair<Rule>) -> Result<String, Error> {
  match build_ast_from_noun(pair) {
    Ok(AstNode::Name(n)) => Ok(n),
    Ok(e) => {
      let e = e.to_string();
      Err(Error::InvalidNoun("<name>".to_string(), e.to_string()))
    }
    Err(e) => Err(e),
  }
}

fn parse_fn_expr(args: Option<Vec<String>>, expr: AstNode) -> Result<AstNode, Error> {
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

fn parse_fn_call(name: String, args: Option<Vec<AstNode>>) -> Result<AstNode, Error> {
  Ok(AstNode::FnCall { name, args })
}

fn build_ast_from_noun(pair: pest::iterators::Pair<Rule>) -> Result<AstNode, Error> {
  match pair.as_rule() {
    Rule::int => {
      let istr = pair.as_str();
      let (sign, istr) = match &istr[..1] {
        "-" => (-1, &istr[1..]),
        _ => (1, &istr[..]),
      };
      let integer: i64 = istr.parse().unwrap();
      Ok(AstNode::Int(sign * integer))
    }
    Rule::decimal => {
      let dstr = pair.as_str();
      let (sign, dstr) = match &dstr[..1] {
        "-" => (-1.0, &dstr[1..]),
        _ => (1.0, &dstr[..]),
      };
      let mut flt: f64 = dstr.parse().unwrap();
      if flt != 0.0 {
        // Avoid negative zeroes; only multiply sign by nonzeroes.
        flt *= sign;
      }
      Ok(AstNode::Float(flt))
    }
    Rule::string => {
      let str = &pair.as_str();
      // Strip leading and ending quotes.
      let str = &str[1..str.len() - 1];
      // Escaped string quotes become single quotes here.
      let str = str.replace("\"\"", "\"");
      Ok(AstNode::Str(String::from(&str[..])))
    }
    Rule::symbol => {
      // strip leading backtick character
      let sym = pair.as_str().strip_prefix("`").unwrap();
      Ok(AstNode::Symbol(String::from(sym)))
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
        let key = match build_ast_from_noun(pair.next().unwrap()) {
          Ok(AstNode::Name(k)) => Some(k),
          _ => None,
        };

        match pair.next() {
          Some(s) => {
            if s.as_str() == ":" {
              let val = pair.next().unwrap();
              let val = match build_ast_from_expr(val) {
                Ok(v) => Some(v),
                e => None,
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
        let col = match build_ast_from_noun(pair.next().unwrap()) {
          Ok(AstNode::Name(c)) => Some(c),
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
    Rule::name => Ok(AstNode::Name(String::from(pair.as_str()))),
    _ => Err(Error::PestErr(PestError::new_from_span(
      ErrorVariant::CustomError {
        message: "invalid noun".to_string(),
      },
      pair.as_span(),
    ))),
  }
}
