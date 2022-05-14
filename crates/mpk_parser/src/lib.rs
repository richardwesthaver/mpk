//! MPK_AST
#[macro_use]
extern crate pest_derive;

use std::ffi::CString;

use pest::error::Error;
use pest::Parser;

pub mod arena;
pub mod ast;
use ast::*;

#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct MpkParser;

pub fn parse(source: &str) -> Result<Program, Error<Rule>> {
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

fn build_ast_from_expr(
  pair: pest::iterators::Pair<Rule>,
) -> Result<AstNode, Error<Rule>> {
  match pair.as_rule() {
    Rule::expr => build_ast_from_expr(pair.into_inner().next().unwrap()),
    Rule::dyadicExpr => {
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
    Rule::monadicExpr => {
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
      // wrapping it in a Nouns node.
      match nouns.len() {
        1 => Ok(nouns.get(0).unwrap().clone()),
        _ => Ok(AstNode::Nouns(nouns)),
      }
    }
    Rule::assgmtExpr => {
      let mut pair = pair.into_inner();
      let name = pair.next().unwrap();
      let expr = pair.next().unwrap();
      let expr = build_ast_from_expr(expr)?;
      Ok(AstNode::IsGlobal {
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
    _ => Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "unexpected expression".to_string(),
      },
      pair.as_span(),
    )),
  }
}

fn parse_dyadic_verb(
  pair: pest::iterators::Pair<Rule>,
  adverb: Option<AdVerb>,
  lhs: AstNode,
  rhs: AstNode,
) -> Result<AstNode, Error<Rule>> {
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
    Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "invalid dyadic verb".to_string(),
      },
      pair.as_span(),
    ))
  }
}

fn parse_monadic_verb(
  pair: pest::iterators::Pair<Rule>,
  adverb: Option<AdVerb>,
  expr: AstNode,
) -> Result<AstNode, Error<Rule>> {
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
    Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "invalid monadic verb".to_string(),
      },
      pair.as_span(),
    ))
  }
}

fn parse_ad_verb(
  pair: pest::iterators::Pair<Rule>,
) -> Result<Option<AdVerb>, Error<Rule>> {
  match pair.as_str() {
    "'" => Ok(Some(AdVerb::Each)),
    "/" => Ok(Some(AdVerb::Over)),
    "\\" => Ok(Some(AdVerb::Scan)),
    "':" => Ok(Some(AdVerb::EachPrior)),
    "\\:" => Ok(Some(AdVerb::EachLeft)),
    "/:" => Ok(Some(AdVerb::EachRight)),
    _ => Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "invalid adverb".to_string(),
      },
      pair.as_span(),
    )),
  }
}

fn parse_sys_verb(
  pair: pest::iterators::Pair<Rule>,
  args: Option<AstNode>,
) -> Result<AstNode, Error<Rule>> {
  let verb = match pair.as_str().strip_prefix("\\").unwrap() {
    "sesh" => Ok(SysVerb::Sesh),
    "http" => Ok(SysVerb::Http),
    "osc" => Ok(SysVerb::Osc),
    "db" => Ok(SysVerb::Db),
    e => Err(format!("invalid sys verb: {}", e.to_string())),
  };
  if verb.is_ok() {
    Ok(AstNode::SysFn {
      verb: verb.unwrap(),
      args: args.map(|e| Box::new(e)),
    })
  } else {
    Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "invalid sys verb".to_string(),
      },
      pair.as_span(),
    ))
  }
}

fn build_ast_from_noun(
  pair: pest::iterators::Pair<Rule>,
) -> Result<AstNode, Error<Rule>> {
  match pair.as_rule() {
    Rule::int => {
      let istr = pair.as_str();
      let (sign, istr) = match &istr[..1] {
        "-" => (-1, &istr[1..]),
        _ => (1, &istr[..]),
      };
      let integer: i32 = istr.parse().unwrap();
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
      Ok(AstNode::Str(CString::new(&str[..]).unwrap()))
    }
    Rule::symbol => {
      let sym = pair.as_str().strip_prefix("`").unwrap();
      Ok(AstNode::Symbol(String::from(sym)))
    }
    Rule::expr => build_ast_from_expr(pair),
    Rule::name => Ok(AstNode::Name(String::from(pair.as_str()))),
    _ => Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "invalid noun".to_string(),
      },
      pair.as_span(),
    )),
  }
}

#[cfg(test)]
mod tests {}
