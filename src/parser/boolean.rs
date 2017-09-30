named!(pub boolean_literal<bool>,
    alt!(
        value!(true, tag!("true")) |
        value!(false, tag!("false"))
    )
);
