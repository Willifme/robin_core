use ast::Expression;

// Such imports are used to allow for the expr_literal macro to work
use parser::number::numeric_literal;
use parser::identifier::identifier_literal;

// Temp do bad stuff
named!(pub expression_literal<Expression>, 
    complete!(alt!(numeric_literal | identifier_literal))
);