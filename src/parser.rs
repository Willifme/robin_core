// auto-generated: "lalrpop 0.15.1"
use std::f64;
use std::str::FromStr;
use ast::{Expression, NumberExpression, StringExpression, BooleanExpression, IdentifierExpression, ListExpression};
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Exprs {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::f64;
    use std::str::FromStr;
    use ast::{Expression, NumberExpression, StringExpression, BooleanExpression, IdentifierExpression, ListExpression};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Variant0(&'input str),
        Variant1(Expression),
        Variant2(Box<Expression>),
        Variant3(::std::vec::Vec<Box<Expression>>),
        Variant4(String),
        Variant5(::std::vec::Vec<Expression>),
        Variant6(Vec<Expression>),
    }
    const __ACTION: &'static [i8] = &[
        // State 0
        16, 17, 0, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
        // State 1
        -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33, -33,
        // State 2
        -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14, -14,
        // State 3
        -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30, -30,
        // State 4
        -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9, -9,
        // State 5
        -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10, -10,
        // State 6
        -20, -20, 0, -20, -20, -20, -20, -20, -20, -20, -20, -20, -20,
        // State 7
        16, 17, 0, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31, -31,
        // State 10
        -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17, -17,
        // State 11
        -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16,
        // State 12
        -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13, -13,
        // State 13
        -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32, -32,
        // State 14
        -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15, -15,
        // State 15
        0, 29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        16, 17, 33, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
        // State 17
        -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3, -3,
        // State 18
        -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2, -2,
        // State 19
        -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35, -35,
        // State 20
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        // State 21
        -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34, -34,
        // State 22
        -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24, -24,
        // State 23
        -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11, -11,
        // State 24
        -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12, -12,
        // State 25
        -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25, -25,
        // State 26
        -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36, -36,
        // State 27
        -21, -21, 0, -21, -21, -21, -21, -21, -21, -21, -21, -21, -21,
        // State 28
        16, 17, 35, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
        // State 29
        -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7, -7,
        // State 30
        16, 17, 37, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
        // State 31
        -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4, -4,
        // State 32
        -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28, -28,
        // State 33
        16, 17, 38, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
        // State 34
        -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26, -26,
        // State 35
        -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8, -8,
        // State 36
        -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29, -29,
        // State 37
        -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27, -27,
    ];
    const __EOF_ACTION: &'static [i8] = &[
        // State 0
        -22,
        // State 1
        -33,
        // State 2
        -14,
        // State 3
        -30,
        // State 4
        -9,
        // State 5
        -10,
        // State 6
        -20,
        // State 7
        -23,
        // State 8
        -37,
        // State 9
        -31,
        // State 10
        -17,
        // State 11
        -16,
        // State 12
        -13,
        // State 13
        -32,
        // State 14
        -15,
        // State 15
        0,
        // State 16
        0,
        // State 17
        -3,
        // State 18
        -2,
        // State 19
        -35,
        // State 20
        -1,
        // State 21
        -34,
        // State 22
        -24,
        // State 23
        -11,
        // State 24
        -12,
        // State 25
        -25,
        // State 26
        -36,
        // State 27
        -21,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        -28,
        // State 33
        0,
        // State 34
        -26,
        // State 35
        0,
        // State 36
        -29,
        // State 37
        -27,
    ];
    const __GOTO: &'static [i8] = &[
        // State 0
        2, 3, 0, 0, 0, 4, 5, 6, 7, 0, 8, 9, 10, 11, 12, 13, 14, 15, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        2, 3, 0, 0, 0, 4, 5, 6, 28, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        2, 3, 30, 0, 31, 4, 5, 6, 32, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        2, 3, 30, 0, 34, 4, 5, 6, 32, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        2, 3, 36, 0, 0, 4, 5, 6, 32, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 33
        2, 3, 36, 0, 0, 4, 5, 6, 32, 0, 0, 0, 10, 11, 12, 13, 14, 15, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""\'""###,
            r###""(""###,
            r###"")""###,
            r###""false""###,
            r###""true""###,
            r###"r#"\"[^\"]*\""#"###,
            r###"r#"0(b|B)[0..1]*"#"###,
            r###"r#"0[oO][0-7]*"#"###,
            r###"r#"0[xX][0-9A-F]*"#"###,
            r###"r#"[0-9]+"#"###,
            r###"r#"[0-9]+([eE][0-9]+)"#"###,
            r###"r#"[a-zA-Z.;,<>=!+\\-&*|^~?:/_][a-zA-Z.;,<>=!+\\-&*|^~?:/_0-9]*"#"###,
            r###"r#"`[^\"]*`"#"###,
        ];
        __ACTION[(__state * 13)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    pub struct ExprsParser {
        builder: super::__intern_token::__MatcherBuilder,
        _priv: (),
    }

    impl ExprsParser {
        pub fn new() -> ExprsParser {
            let __builder = super::__intern_token::__MatcherBuilder::new();
            ExprsParser {
                builder: __builder,
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
        >(
            &self,
            input: &'input str,
        ) -> Result<Vec<Expression>, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
        {
            let mut __tokens = self.builder.matcher(input);
            let mut __states = vec![0_i8];
            let mut __symbols = vec![];
            let mut __integer;
            let mut __lookahead;
            let __last_location = &mut Default::default();
            '__shift: loop {
                __lookahead = match __tokens.next() {
                    Some(Ok(v)) => v,
                    None => break '__shift,
                    Some(Err(e)) => return Err(e),
                };
                *__last_location = __lookahead.2.clone();
                __integer = match __lookahead.1 {
                    Token(8, _) if true => 0,
                    Token(9, _) if true => 1,
                    Token(10, _) if true => 2,
                    Token(11, _) if true => 3,
                    Token(12, _) if true => 4,
                    Token(0, _) if true => 5,
                    Token(1, _) if true => 6,
                    Token(2, _) if true => 7,
                    Token(3, _) if true => 8,
                    Token(4, _) if true => 9,
                    Token(5, _) if true => 10,
                    Token(6, _) if true => 11,
                    Token(7, _) if true => 12,
                    _ => {
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: Some(__lookahead),
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error);
                    }
                };
                '__inner: loop {
                    let __state = *__states.last().unwrap() as usize;
                    let __action = __ACTION[__state * 13 + __integer];
                    if __action > 0 {
                        let __symbol = match __integer {
                            0 => match __lookahead.1 {
                                Token(8, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            1 => match __lookahead.1 {
                                Token(9, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            2 => match __lookahead.1 {
                                Token(10, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            3 => match __lookahead.1 {
                                Token(11, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            4 => match __lookahead.1 {
                                Token(12, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            5 => match __lookahead.1 {
                                Token(0, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            6 => match __lookahead.1 {
                                Token(1, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            7 => match __lookahead.1 {
                                Token(2, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            8 => match __lookahead.1 {
                                Token(3, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            9 => match __lookahead.1 {
                                Token(4, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            10 => match __lookahead.1 {
                                Token(5, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            11 => match __lookahead.1 {
                                Token(6, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            12 => match __lookahead.1 {
                                Token(7, __tok0) => __Symbol::Variant0((__tok0)),
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        };
                        __states.push(__action - 1);
                        __symbols.push((__lookahead.0, __symbol, __lookahead.2));
                        continue '__shift;
                    } else if __action < 0 {
                        if let Some(r) = __reduce(input, __action, Some(&__lookahead.0), &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                            if r.is_err() {
                                return r;
                            }
                            return Err(__lalrpop_util::ParseError::ExtraToken { token: __lookahead });
                        }
                    } else {
                        let mut __err_lookahead = Some(__lookahead);
                        let mut __err_integer: Option<usize> = Some(__integer);
                        let __state = *__states.last().unwrap() as usize;
                        let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                            token: __err_lookahead,
                            expected: __expected_tokens(__state),
                        };
                        return Err(__error)
                    }
                }
            }
            loop {
                let __state = *__states.last().unwrap() as usize;
                let __action = __EOF_ACTION[__state];
                if __action < 0 {
                    if let Some(r) = __reduce(input, __action, None, &mut __states, &mut __symbols, ::std::marker::PhantomData::<()>) {
                        return r;
                    }
                } else {
                    let mut __err_lookahead = None;
                    let mut __err_integer: Option<usize> = None;
                    let __state = *__states.last().unwrap() as usize;
                    let __error = __lalrpop_util::ParseError::UnrecognizedToken {
                        token: __err_lookahead,
                        expected: __expected_tokens(__state),
                    };
                    return Err(__error)
                }
            }
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i8,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i8>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<Vec<Expression>,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let (__pop_states, __symbol, __nonterminal) = match -__action {
            1 => {
                (|| {
                    // BinaryDigit = r#"0(b|B)[0..1]*"# => ActionFn(19);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action19::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 0)
                })()
            }
            2 => {
                (|| {
                    // Boolean = "true" => ActionFn(15);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action15::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 1)
                })()
            }
            3 => {
                (|| {
                    // Boolean = "false" => ActionFn(16);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action16::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 1)
                })()
            }
            4 => {
                (|| {
                    // Boxed_Expr = Expr => ActionFn(9);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action9::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant2(__nt), __end);
                    (1, __symbol, 2)
                })()
            }
            5 => {
                (|| {
                    // Boxed_Expr* =  => ActionFn(26);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action26::<>(input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (0, __symbol, 3)
                })()
            }
            6 => {
                (|| {
                    // Boxed_Expr* = Boxed_Expr+ => ActionFn(27);
                    let __sym0 = __pop_Variant3(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action27::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (1, __symbol, 3)
                })()
            }
            7 => {
                (|| {
                    // Boxed_Expr+ = Boxed_Expr => ActionFn(32);
                    let __sym0 = __pop_Variant2(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action32::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (1, __symbol, 4)
                })()
            }
            8 => {
                (|| {
                    // Boxed_Expr+ = Boxed_Expr+, Boxed_Expr => ActionFn(33);
                    let __sym1 = __pop_Variant2(__symbols);
                    let __sym0 = __pop_Variant3(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action33::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant3(__nt), __end);
                    (2, __symbol, 4)
                })()
            }
            9 => {
                (|| {
                    // Decimal = Decimal_Digit => ActionFn(22);
                    let __sym0 = __pop_Variant4(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action22::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 5)
                })()
            }
            10 => {
                (|| {
                    // Decimal = ExponentPart => ActionFn(23);
                    let __sym0 = __pop_Variant4(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action23::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 5)
                })()
            }
            11 => {
                (|| {
                    // Decimal_Digit = r#"[0-9]+"# => ActionFn(25);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action25::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (1, __symbol, 6)
                })()
            }
            12 => {
                (|| {
                    // ExponentPart = r#"[0-9]+([eE][0-9]+)"# => ActionFn(24);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action24::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant4(__nt), __end);
                    (1, __symbol, 7)
                })()
            }
            13 => {
                (|| {
                    // Expr = Number => ActionFn(2);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action2::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            14 => {
                (|| {
                    // Expr = Boolean => ActionFn(3);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action3::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            15 => {
                (|| {
                    // Expr = String => ActionFn(4);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action4::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            16 => {
                (|| {
                    // Expr = List => ActionFn(5);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action5::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            17 => {
                (|| {
                    // Expr = Identifier => ActionFn(6);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action6::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 8)
                })()
            }
            18 => {
                (|| {
                    // Expr* =  => ActionFn(28);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action28::<>(input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (0, __symbol, 9)
                })()
            }
            19 => {
                (|| {
                    // Expr* = Expr+ => ActionFn(29);
                    let __sym0 = __pop_Variant5(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action29::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 9)
                })()
            }
            20 => {
                (|| {
                    // Expr+ = Expr => ActionFn(30);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action30::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (1, __symbol, 10)
                })()
            }
            21 => {
                (|| {
                    // Expr+ = Expr+, Expr => ActionFn(31);
                    let __sym1 = __pop_Variant1(__symbols);
                    let __sym0 = __pop_Variant5(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action31::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant5(__nt), __end);
                    (2, __symbol, 10)
                })()
            }
            22 => {
                (|| {
                    // Exprs =  => ActionFn(38);
                    let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                    let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                    let __nt = super::__action38::<>(input, &__start, &__end);
                    let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                    (0, __symbol, 11)
                })()
            }
            23 => {
                (|| {
                    // Exprs = Expr+ => ActionFn(39);
                    let __sym0 = __pop_Variant5(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action39::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant6(__nt), __end);
                    (1, __symbol, 11)
                })()
            }
            24 => {
                (|| {
                    // HexDigit = r#"0[xX][0-9A-F]*"# => ActionFn(21);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action21::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 12)
                })()
            }
            25 => {
                (|| {
                    // Identifier = r#"[a-zA-Z.;,<>=!+\\-&*|^~?:/_][a-zA-Z.;,<>=!+\\-&*|^~?:/_0-9]*"# => ActionFn(10);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action10::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 13)
                })()
            }
            26 => {
                (|| {
                    // List = "\'", "(", ")" => ActionFn(34);
                    let __sym2 = __pop_Variant0(__symbols);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action34::<>(input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (3, __symbol, 14)
                })()
            }
            27 => {
                (|| {
                    // List = "\'", "(", Boxed_Expr+, ")" => ActionFn(35);
                    let __sym3 = __pop_Variant0(__symbols);
                    let __sym2 = __pop_Variant3(__symbols);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym3.2.clone();
                    let __nt = super::__action35::<>(input, __sym0, __sym1, __sym2, __sym3);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (4, __symbol, 14)
                })()
            }
            28 => {
                (|| {
                    // List = "(", ")" => ActionFn(36);
                    let __sym1 = __pop_Variant0(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym1.2.clone();
                    let __nt = super::__action36::<>(input, __sym0, __sym1);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (2, __symbol, 14)
                })()
            }
            29 => {
                (|| {
                    // List = "(", Boxed_Expr+, ")" => ActionFn(37);
                    let __sym2 = __pop_Variant0(__symbols);
                    let __sym1 = __pop_Variant3(__symbols);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym2.2.clone();
                    let __nt = super::__action37::<>(input, __sym0, __sym1, __sym2);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (3, __symbol, 14)
                })()
            }
            30 => {
                (|| {
                    // Number = Decimal => ActionFn(11);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action11::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 15)
                })()
            }
            31 => {
                (|| {
                    // Number = HexDigit => ActionFn(12);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action12::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 15)
                })()
            }
            32 => {
                (|| {
                    // Number = OctDigit => ActionFn(13);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action13::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 15)
                })()
            }
            33 => {
                (|| {
                    // Number = BinaryDigit => ActionFn(14);
                    let __sym0 = __pop_Variant1(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action14::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 15)
                })()
            }
            34 => {
                (|| {
                    // OctDigit = r#"0[oO][0-7]*"# => ActionFn(20);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action20::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 16)
                })()
            }
            35 => {
                (|| {
                    // String = r#"\"[^\"]*\""# => ActionFn(17);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action17::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 17)
                })()
            }
            36 => {
                (|| {
                    // String = r#"`[^\"]*`"# => ActionFn(18);
                    let __sym0 = __pop_Variant0(__symbols);
                    let __start = __sym0.0.clone();
                    let __end = __sym0.2.clone();
                    let __nt = super::__action18::<>(input, __sym0);
                    let __symbol = (__start, __Symbol::Variant1(__nt), __end);
                    (1, __symbol, 17)
                })()
            }
            37 => {
                // __Exprs = Exprs => ActionFn(0);
                let __sym0 = __pop_Variant6(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        __symbols.push(__symbol);
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 19 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Variant2<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Box<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant2(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant1<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Expression, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant1(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant4<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, String, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant4(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant6<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant6(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant3<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Box<Expression>>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant3(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant5<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<Expression>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant5(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Variant0<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Variant0(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Exprs::ExprsParser;
#[cfg_attr(rustfmt, rustfmt_skip)]
mod __intern_token {
    #![allow(unused_imports)]
    use std::f64;
    use std::str::FromStr;
    use ast::{Expression, NumberExpression, StringExpression, BooleanExpression, IdentifierExpression, ListExpression};
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    extern crate regex as __regex;
    use std::fmt as __fmt;

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub struct Token<'input>(pub usize, pub &'input str);
    impl<'a> __fmt::Display for Token<'a> {
        fn fmt(&self, formatter: &mut __fmt::Formatter) -> Result<(), __fmt::Error> {
            __fmt::Display::fmt(self.1, formatter)
        }
    }

    pub struct __MatcherBuilder {
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl __MatcherBuilder {
        pub fn new() -> __MatcherBuilder {
            let __strs: &[&str] = &[
                "^((?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:\"))",
                "^((?u:0)((?u:b)|(?u:B))(?u:[\\.-\\.0-1])*)",
                "^((?u:0)(?u:[O-Oo-o])(?u:[0-7])*)",
                "^((?u:0)(?u:[X-Xx-x])(?u:[0-9A-F])*)",
                "^((?u:[0-9])+)",
                "^((?u:[0-9])+((?u:[E-Ee-e])(?u:[0-9])+))",
                "^((?u:[!-!\\&-\\&\\*-/:-\\?A-Z\\^-_a-z\\|-\\|\\~-\\~])(?u:[!-!\\&-\\&\\*-\\?A-Z\\^-_a-z\\|-\\|\\~-\\~])*)",
                "^((?u:`)(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:`))",
                "^((?u:\'))",
                "^((?u:\\())",
                "^((?u:\\)))",
                "^((?u:false))",
                "^((?u:true))",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^((?u:\")(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:\"))").unwrap(),
                __regex::Regex::new("^((?u:0)((?u:b)|(?u:B))(?u:[\\.-\\.0-1])*)").unwrap(),
                __regex::Regex::new("^((?u:0)(?u:[O-Oo-o])(?u:[0-7])*)").unwrap(),
                __regex::Regex::new("^((?u:0)(?u:[X-Xx-x])(?u:[0-9A-F])*)").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+)").unwrap(),
                __regex::Regex::new("^((?u:[0-9])+((?u:[E-Ee-e])(?u:[0-9])+))").unwrap(),
                __regex::Regex::new("^((?u:[!-!\\&-\\&\\*-/:-\\?A-Z\\^-_a-z\\|-\\|\\~-\\~])(?u:[!-!\\&-\\&\\*-\\?A-Z\\^-_a-z\\|-\\|\\~-\\~])*)").unwrap(),
                __regex::Regex::new("^((?u:`)(?u:[\u{0}-!\\#-\u{10ffff}])*(?u:`))").unwrap(),
                __regex::Regex::new("^((?u:\'))").unwrap(),
                __regex::Regex::new("^((?u:\\())").unwrap(),
                __regex::Regex::new("^((?u:\\)))").unwrap(),
                __regex::Regex::new("^((?u:false))").unwrap(),
                __regex::Regex::new("^((?u:true))").unwrap(),
            ];
            __MatcherBuilder { regex_set: __regex_set, regex_vec: __regex_vec }
        }
        pub fn matcher<'input, 'builder>(&'builder self, s: &'input str) -> __Matcher<'input, 'builder> {
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: &self.regex_set,
                regex_vec: &self.regex_vec,
            }
        }
    }

    pub struct __Matcher<'input, 'builder> {
        text: &'input str,
        consumed: usize,
        regex_set: &'builder __regex::RegexSet,
        regex_vec: &'builder Vec<__regex::Regex>,
    }

    impl<'input, 'builder> Iterator for __Matcher<'input, 'builder> {
        type Item = Result<(usize, Token<'input>, usize), __lalrpop_util::ParseError<usize,Token<'input>,&'static str>>;

        fn next(&mut self) -> Option<Self::Item> {
            let __text = self.text.trim_left();
            let __whitespace = self.text.len() - __text.len();
            let __start_offset = self.consumed + __whitespace;
            if __text.is_empty() {
                self.text = __text;
                self.consumed = __start_offset;
                None
            } else {
                let __matches = self.regex_set.matches(__text);
                if !__matches.matched_any() {
                    Some(Err(__lalrpop_util::ParseError::InvalidToken {
                        location: __start_offset,
                    }))
                } else {
                    let mut __longest_match = 0;
                    let mut __index = 0;
                    for __i in 0 .. 13 {
                        if __matches.matched(__i) {
                            let __match = self.regex_vec[__i].find(__text).unwrap();
                            let __len = __match.end();
                            if __len >= __longest_match {
                                __longest_match = __len;
                                __index = __i;
                            }
                        }
                    }
                    let __result = &__text[..__longest_match];
                    let __remaining = &__text[__longest_match..];
                    let __end_offset = __start_offset + __longest_match;
                    self.text = __remaining;
                    self.consumed = __end_offset;
                    Some(Ok((__start_offset, Token(__index, __result), __end_offset)))
                }
            }
        }
    }
}
pub use self::__intern_token::Token;

#[allow(unused_variables)]
fn __action0<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<Expression>, usize),
) -> Vec<Expression>
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ::std::vec::Vec<Expression>, usize),
) -> Vec<Expression>
{
    __0
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, exprs, _): (usize, ::std::vec::Vec<Box<Expression>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::List(ListExpression::new_quoted(exprs))
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, exprs, _): (usize, ::std::vec::Vec<Box<Expression>>, usize),
    (_, _, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::List(ListExpression::new_unquoted(exprs))
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Box<Expression>
{
    Box::new(__0)
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Identifier(IdentifierExpression::new(__0.to_string()))
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> Expression
{
    (__0)
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Boolean(BooleanExpression::new(true))
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Boolean(BooleanExpression::new(false))
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::String(StringExpression::new(__0.to_string()))
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::String(StringExpression::new(__0.to_string()))
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Number(NumberExpression::new(f64::from(i32::from_str_radix(&__0[2..], 2).unwrap())))
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Number(NumberExpression::new(f64::from(i32::from_str_radix(&__0[2..], 8).unwrap())))
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> Expression
{
    Expression::Number(NumberExpression::new(f64::from(i32::from_str_radix(&__0[2..], 16).unwrap())))
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Expression
{
    Expression::Number(NumberExpression::new(f64::from_str(&__0).unwrap()))
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, String, usize),
) -> Expression
{
    Expression::Number(NumberExpression::new(f64::from_str(&__0).unwrap()))
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> String
{
    __0.to_string()
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Box<Expression>>
{
    vec![]
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expression>>, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    v
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<Expression>
{
    vec![]
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expression>, usize),
) -> ::std::vec::Vec<Expression>
{
    v
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Expression, usize),
) -> ::std::vec::Vec<Expression>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Expression>, usize),
    (_, e, _): (usize, Expression, usize),
) -> ::std::vec::Vec<Expression>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Box<Expression>, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<Box<Expression>>, usize),
    (_, e, _): (usize, Box<Expression>, usize),
) -> ::std::vec::Vec<Box<Expression>>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, &'input str, usize),
) -> Expression
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
    __2: (usize, ::std::vec::Vec<Box<Expression>>, usize),
    __3: (usize, &'input str, usize),
) -> Expression
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action27(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action7(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, &'input str, usize),
) -> Expression
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action26(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    __0: (usize, &'input str, usize),
    __1: (usize, ::std::vec::Vec<Box<Expression>>, usize),
    __2: (usize, &'input str, usize),
) -> Expression
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action27(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action8(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<Expression>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action28(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<Expression>, usize),
) -> Vec<Expression>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action29(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        input,
        __temp0,
    )
}

pub trait __ToTriple<'input, > {
    type Error;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),Self::Error>;
}

impl<'input, > __ToTriple<'input, > for (usize, Token<'input>, usize) {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        Ok(value)
    }
}
impl<'input, > __ToTriple<'input, > for Result<(usize, Token<'input>, usize),&'static str> {
    type Error = &'static str;
    fn to_triple(value: Self) -> Result<(usize,Token<'input>,usize),&'static str> {
        value
    }
}
