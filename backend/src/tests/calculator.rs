#![allow(unused)]
use rstest::rstest;

use crate::domain::calculator::evaluate_expression;

#[rstest]
#[case("3", "3")]
#[case("1+2", "3")]
#[case("1/2", "0.5")]
#[case("1*2", "2")]
#[case("2*3+10/2", "11")]
#[case("(3+2)*3", "15")]
#[case("((((1)*2)*3)*4)*5", "120")]
#[case("1-10", "-9")]
#[case("-10*30*(-2+10)", "-2400")]
#[case("-------32", "-32")]
#[case("------32", "32")]
#[case(
    "((42+17)*(93-58)/(7+3)+(81*(14+6)-55)/(9-4))+((64*(23-11)+75)/(8+2)-(19*(45-33)/\
     (6+3)))+(((12+34)*(56-21))/(9+5)+(78*(90-67))/(4+8))-((51*(72-48)+(63+29)*7)/(6+5))+((88+13)*\
     (27-19)/(3+1)+(46*(59-41)/(7+2)))+(((15+25)*(35+45)-(55*65))/(5+10))+((91*
    (82-73)+(64+28)*(14-7))/(6+9))-((33*(44+55)-(66*11))/(8+4))+(((71+29)*(38-17))/(9+3)+(52*\
     (63-41))/(5+5))",
    "1117.331818181818"
)]
#[case("", "Unexpected token: Eof")]
#[case("1-", "Unexpected token: Eof")]
#[case("(3-2", "Expected closing parenthesis")]
#[case("1+1(", "Unexpected token: LPar")]
#[case("1+1)", "Unexpected token: RPar")]
#[case("(1+1)/(1-1)", "division by zero")]
#[case("1.23L6", "Unexpected token: Some('L')")]
fn test_expr_parser(#[case] expr: &str, #[case] expected: &str) {
    // let result = calculate(query).await;
    match evaluate_expression(expr) {
        Ok(res) => assert_eq!(res.to_string(), expected),
        Err(err) => assert_eq!(err.to_string(), expected),
    }
}
