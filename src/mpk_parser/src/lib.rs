//! MPK_AST
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
use std::ffi::CString;

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
    Rule::monadicExpr => {
      let mut pair = pair.into_inner();
      let verb = pair.next().unwrap();
      let expr = pair.next().unwrap();
      let expr = build_ast_from_expr(expr)?;
      parse_monadic_verb(verb, expr)
    }
    Rule::dyadicExpr => {
      let mut pair = pair.into_inner();
      let lhspair = pair.next().unwrap();
      let lhs = build_ast_from_expr(lhspair)?;
      let verb = pair.next().unwrap();
      let rhspair = pair.next().unwrap();
      let rhs = build_ast_from_expr(rhspair)?;
      parse_dyadic_verb(verb, lhs, rhs)
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
      let ident = pair.next().unwrap();
      let expr = pair.next().unwrap();
      let expr = build_ast_from_expr(expr)?;
      Ok(AstNode::IsGlobal {
        ident: String::from(ident.as_str()),
        expr: Box::new(expr),
      })
    }
    Rule::sysExpr => {
      let mut pair = pair.into_inner();
      let verb = pair.next().unwrap();
      let expr = if let Some(e) = pair.next() {
        Some(build_ast_from_expr(e)?)
      } else {
        None
      };
      parse_sys_verb(verb, expr)
    }
    _ => Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "Unexpected expression".to_string(),
      },
      pair.as_span(),
    )),
  }
}

fn parse_dyadic_verb(
  pair: pest::iterators::Pair<Rule>,
  lhs: AstNode,
  rhs: AstNode,
) -> Result<AstNode, Error<Rule>> {
  let verb = match pair.as_str() {
    "+" => Ok(DyadicVerb::Plus),
    "*" => Ok(DyadicVerb::Times),
    "-" => Ok(DyadicVerb::Minus),
    "<" => Ok(DyadicVerb::LessThan),
    "=" => Ok(DyadicVerb::Equal),
    ">" => Ok(DyadicVerb::LargerThan),
    "%" => Ok(DyadicVerb::Divide),
    "^" => Ok(DyadicVerb::Power),
    "|" => Ok(DyadicVerb::Residue),
    "#" => Ok(DyadicVerb::Copy),
    ">." => Ok(DyadicVerb::LargerOf),
    ">:" => Ok(DyadicVerb::LargerOrEqual),
    "$" => Ok(DyadicVerb::Shape),
    e => Err(format!("Unexpected dyadic verb: {}", e.to_string())),
  };
  if verb.is_ok() {
    Ok(AstNode::DyadicOp {
      lhs: Box::new(lhs),
      rhs: Box::new(rhs),
      verb: verb.unwrap(),
    })
  } else {
    Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "Unsupported monadic verb".to_string(),
      },
      pair.as_span(),
    ))
  }
}

fn parse_monadic_verb(
  pair: pest::iterators::Pair<Rule>,
  expr: AstNode,
) -> Result<AstNode, Error<Rule>> {
  let verb = match pair.as_str() {
    ">:" => Ok(MonadicVerb::Increment),
    "*:" => Ok(MonadicVerb::Square),
    "-" => Ok(MonadicVerb::Negate),
    "%" => Ok(MonadicVerb::Reciprocal),
    "#" => Ok(MonadicVerb::Tally),
    ">." => Ok(MonadicVerb::Ceiling),
    "$" => Ok(MonadicVerb::ShapeOf),
    e => Err(format!("Unsupported monadic verb: {}", e.to_string())),
  };
  if verb.is_ok() {
    Ok(AstNode::MonadicOp {
      verb: verb.unwrap(),
      expr: Box::new(expr),
    })
  } else {
    Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "Unsupported monadic verb".to_string(),
      },
      pair.as_span(),
    ))
  }
}

fn parse_sys_verb(
  pair: pest::iterators::Pair<Rule>,
  expr: Option<AstNode>,
) -> Result<AstNode, Error<Rule>> {
  let verb = match pair.as_str() {
    "0:http" => Ok(SysVerb::Http),
    "0:osc" => Ok(SysVerb::Osc),
    "0:sql" => Ok(SysVerb::Sql),
    e => Err(format!("Unsupported sys verb: {}", e.to_string())),
  };
  if verb.is_ok() {
    Ok(AstNode::SysOp {
      verb: verb.unwrap(),
      expr: expr.map(|e| Box::new(e)),
    })
  } else {
    Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "Unsupported monadic verb".to_string(),
      },
      pair.as_span(),
    ))
  }
}

fn build_ast_from_noun(
  pair: pest::iterators::Pair<Rule>,
) -> Result<AstNode, Error<Rule>> {
  match pair.as_rule() {
    Rule::integer => {
      let istr = pair.as_str();
      let (sign, istr) = match &istr[..1] {
        "_" => (-1, &istr[1..]),
        _ => (1, &istr[..]),
      };
      let integer: i32 = istr.parse().unwrap();
      Ok(AstNode::Integer(sign * integer))
    }
    Rule::decimal => {
      let dstr = pair.as_str();
      let (sign, dstr) = match &dstr[..1] {
        "_" => (-1.0, &dstr[1..]),
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
    Rule::expr => build_ast_from_expr(pair),
    Rule::ident => Ok(AstNode::Ident(String::from(pair.as_str()))),
    _ => Err(Error::new_from_span(
      pest::error::ErrorVariant::CustomError {
        message: "Unexpected noun".to_string(),
      },
      pair.as_span(),
    )),
  }
}

#[cfg(test)]
mod tests {}
