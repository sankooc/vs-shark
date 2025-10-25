use anyhow::{bail, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum BpfExpr {
    Ip,
    Tcp,
    Udp,
    Port(u16),
    TcpPort(u16),
    UdpPort(u16),
    And(Box<BpfExpr>, Box<BpfExpr>),
    Or(Box<BpfExpr>, Box<BpfExpr>),
}

pub fn compile_expression(expr: &str) -> Result<BpfExpr> {
    let tokens: Vec<&str> = expr.split_whitespace().collect();
    if tokens.is_empty() {
        bail!("Expression cannot be empty");
    }
    match tokens.as_slice() {
        ["ip"] => Ok(BpfExpr::Ip),
        ["tcp"] => Ok(BpfExpr::Tcp),
        ["udp"] => Ok(BpfExpr::Udp),
        ["port", port] => Ok(BpfExpr::Port(port.parse()?)),
        ["tcp", "port", port] => Ok(BpfExpr::TcpPort(port.parse()?)),
        ["udp", "port", port] => Ok(BpfExpr::UdpPort(port.parse()?)),
        // 支持简单的 and/or 组合
        [left, "and", right @ ..] => {
            let left_expr = compile_expression(left)?;
            let right_expr = compile_expression(&right.join(" "))?;
            Ok(BpfExpr::And(Box::new(left_expr), Box::new(right_expr)))
        }
        [left, "or", right @ ..] => {
            let left_expr = compile_expression(left)?;
            let right_expr = compile_expression(&right.join(" "))?;
            Ok(BpfExpr::Or(Box::new(left_expr), Box::new(right_expr)))
        }
        _ => bail!("Unsupported or invalid BPF expression: {expr}"),
    }
}