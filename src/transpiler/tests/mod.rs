use crate::util;

use super::*;

#[test]
fn print() {
    let input = vec![
        Statement::Call("print".to_string(), vec![Expression::Integer(99)]),
        Statement::Exit,
    ];
    let CodeOutput { code, .. } = transpile(input, None);
    assert_eq!(util::unbleach(code), "sssttsssttntnstnnn");
}
#[test]
fn heap_allocation() {
    let input = vec![
        Statement::IntDeclaration("m".to_string()),
        Statement::Assignment("m".to_string(), Expression::Integer(11)),
        Statement::Exit,
    ];
    let CodeOutput { code, .. } = transpile(input, None);
    assert_eq!(util::unbleach(code), "ssststtnssssnsntttsnnn");
}
#[test]
fn print_from_heap() {
    let input = vec![
        Statement::IntDeclaration("m".to_string()),
        Statement::Assignment("m".to_string(), Expression::Integer(11)),
        Statement::Call(
            "print".to_string(),
            vec![Expression::Variable("m".to_string())],
        ),
        Statement::Exit,
    ];
    let CodeOutput { code, .. } = transpile(input, None);
    assert_eq!(util::unbleach(code), "ssststtnssssnsntttsssssnttttnstnnn");
}
#[test]
fn while_less_than() {
    let input = vec![
        Statement::IntDeclaration("m".to_string()),
        Statement::Assignment("m".to_string(), Expression::Integer(8)),
        Statement::WhileLoop {
            condition: Box::new(Expression::BinaryOp {
                operator: Operation::CompareLessThan,
                left: Box::new(Expression::Variable("m".to_string())),
                right: Box::new(Expression::Integer(11)),
            }),
            body: Box::new(Statement::Block(vec![
                Statement::Call(
                    "print".to_string(),
                    vec![Expression::Variable("m".to_string())],
                ),
                Statement::Assignment(
                    "m".to_string(),
                    Expression::BinaryOp {
                        operator: Operation::Add,
                        left: Box::new(Expression::Variable("m".to_string())),
                        right: Box::new(Expression::Integer(1)),
                    },
                ),
            ])),
        },
        Statement::Exit,
    ];
    let CodeOutput { code, .. } = transpile(input, None);
    assert_eq!(
        util::unbleach(code),
        "ssstsssnssssnsntttsnssnssssntttssststtntsstntttnnsntsnnsstnssssnttttnstssssntttssstntsssssssnsntttsnsnnnsstsnnnn"
    );
}
