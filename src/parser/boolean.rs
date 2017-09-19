use ast::Expression;

named!(pub boolean_literal<Expression>,
    alt!(
        value!(Expression::Boolean(true), tag!("true")) |
        value!(Expression::Boolean(false), tag!("false"))
    )
);
