// Such imports are used to allow for the expr_literal macro to work
use parser::number::numeric_literal;
use parser::identifier::identifier_literal;
use ast::Expression;

// Temp do bad stuff
named!(pub expr_literal<Expression>, 
    alt!(numeric_literal | identifier_literal)
);