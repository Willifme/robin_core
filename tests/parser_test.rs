extern crate robin_core;

extern crate nom;

#[cfg(test)]
mod parser_tests {
    use nom::IResult;

    use robin_core::parser;

    #[test]
    fn it_works() {
        assert_eq!(parser::abcd_parser(b"abcd"), IResult::Done(&b""[..], &b"abcd"[..]))
    }
}