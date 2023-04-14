// auto-generated: "lalrpop 0.19.9"
// sha3: b9f8a4a68f5f10da7f6a8907e9fb55fd66db40cd69079c37a7828edc369b340e
use crate::ast::*;
use crate::token::Token;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;
#[allow(unused_imports)]
use self::__lalrpop_util::state_machine as __state_machine;
extern crate core;
extern crate alloc;

#[cfg_attr(rustfmt, rustfmt_skip)]
mod __parse__Program {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens, clippy::all)]

    use crate::ast::*;
    use crate::token::Token;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    #[allow(unused_imports)]
    use self::__lalrpop_util::state_machine as __state_machine;
    extern crate core;
    extern crate alloc;
    use super::__ToTriple;
    #[allow(dead_code)]
    pub(crate) enum __Symbol<>
     {
        Variant0(Token),
        Variant1(bool),
        Variant2(String),
        Variant3(alloc::vec::Vec<Expr>),
        Variant4(Expr),
    }
    const __ACTION: &[i8] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __action(state: i8, integer: usize) -> i8 {
        __ACTION[(state as usize) * 29 + integer]
    }
    const __EOF_ACTION: &[i8] = &[
        // State 0
        -1,
        // State 1
        -2,
        // State 2
        -3,
        // State 3
        -7,
        // State 4
        -4,
        // State 5
        -8,
    ];
    fn __goto(state: i8, nt: usize) -> i8 {
        match nt {
            0 => 2,
            2 => match state {
                1 => 5,
                _ => 3,
            },
            4 => 1,
            _ => 0,
        }
    }
    fn __expected_tokens(__state: i8) -> alloc::vec::Vec<alloc::string::String> {
        const __TERMINAL: &[&str] = &[
            r###""(""###,
            r###"")""###,
            r###""*""###,
            r###""+""###,
            r###"",""###,
            r###""-""###,
            r###"".""###,
            r###""/""###,
            r###"":""###,
            r###"";""###,
            r###""=""###,
            r###""=""###,
            r###""Bool""###,
            r###""Class""###,
            r###""Else""###,
            r###""Function""###,
            r###""Id""###,
            r###""If""###,
            r###""Inherits""###,
            r###""Int""###,
            r###""Isvoid""###,
            r###""Let""###,
            r###""New""###,
            r###""Not""###,
            r###""String""###,
            r###""TYPE""###,
            r###""While""###,
            r###""{""###,
            r###""}""###,
        ];
        __TERMINAL.iter().enumerate().filter_map(|(index, terminal)| {
            let next_state = __action(__state, index);
            if next_state == 0 {
                None
            } else {
                Some(alloc::string::ToString::to_string(terminal))
            }
        }).collect()
    }
    pub(crate) struct __StateMachine<'input>
    where 
    {
        __phantom: core::marker::PhantomData<(&'input ())>,
    }
    impl<'input> __state_machine::ParserDefinition for __StateMachine<'input>
    where 
    {
        type Location = ();
        type Error = &'static str;
        type Token = Token;
        type TokenIndex = usize;
        type Symbol = __Symbol<>;
        type Success = alloc::vec::Vec<Expr>;
        type StateIndex = i8;
        type Action = i8;
        type ReduceIndex = i8;
        type NonterminalIndex = usize;

        #[inline]
        fn start_location(&self) -> Self::Location {
              Default::default()
        }

        #[inline]
        fn start_state(&self) -> Self::StateIndex {
              0
        }

        #[inline]
        fn token_to_index(&self, token: &Self::Token) -> Option<usize> {
            __token_to_integer(token, core::marker::PhantomData::<(&())>)
        }

        #[inline]
        fn action(&self, state: i8, integer: usize) -> i8 {
            __action(state, integer)
        }

        #[inline]
        fn error_action(&self, state: i8) -> i8 {
            __action(state, 29 - 1)
        }

        #[inline]
        fn eof_action(&self, state: i8) -> i8 {
            __EOF_ACTION[state as usize]
        }

        #[inline]
        fn goto(&self, state: i8, nt: usize) -> i8 {
            __goto(state, nt)
        }

        fn token_to_symbol(&self, token_index: usize, token: Self::Token) -> Self::Symbol {
            __token_to_symbol(token_index, token, core::marker::PhantomData::<(&())>)
        }

        fn expected_tokens(&self, state: i8) -> alloc::vec::Vec<alloc::string::String> {
            __expected_tokens(state)
        }

        #[inline]
        fn uses_error_recovery(&self) -> bool {
            false
        }

        #[inline]
        fn error_recovery_symbol(
            &self,
            recovery: __state_machine::ErrorRecovery<Self>,
        ) -> Self::Symbol {
            panic!("error recovery not enabled for this grammar")
        }

        fn reduce(
            &mut self,
            action: i8,
            start_location: Option<&Self::Location>,
            states: &mut alloc::vec::Vec<i8>,
            symbols: &mut alloc::vec::Vec<__state_machine::SymbolTriple<Self>>,
        ) -> Option<__state_machine::ParseResult<Self>> {
            __reduce(
                action,
                start_location,
                states,
                symbols,
                core::marker::PhantomData::<(&())>,
            )
        }

        fn simulate_reduce(&self, action: i8) -> __state_machine::SimulatedReduce<Self> {
            panic!("error recovery not enabled for this grammar")
        }
    }
    fn __token_to_integer<
        'input,
    >(
        __token: &Token,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<usize>
    {
        match *__token {
            Token::Lparen if true => Some(0),
            Token::Rparen if true => Some(1),
            Token::Mul if true => Some(2),
            Token::Plus if true => Some(3),
            Token::Comma if true => Some(4),
            Token::Minus if true => Some(5),
            Token::Period if true => Some(6),
            Token::Divide if true => Some(7),
            Token::Colon if true => Some(8),
            Token::Semicolon if true => Some(9),
            Token::Equal if true => Some(10),
            Token::Equal if true => Some(11),
            Token::BoolConst(_) if true => Some(12),
            Token::Class_ if true => Some(13),
            Token::Else if true => Some(14),
            Token::Function if true => Some(15),
            Token::Identifier(String) if true => Some(16),
            Token::If if true => Some(17),
            Token::Inherits if true => Some(18),
            Token::IntConst(_) if true => Some(19),
            Token::Isvoid if true => Some(20),
            Token::Let if true => Some(21),
            Token::New if true => Some(22),
            Token::Not if true => Some(23),
            Token::StringConst(_) if true => Some(24),
            Token::TypeId(_) if true => Some(25),
            Token::While if true => Some(26),
            Token::Lbrace if true => Some(27),
            Token::Rbrace if true => Some(28),
            _ => None,
        }
    }
    fn __token_to_symbol<
        'input,
    >(
        __token_index: usize,
        __token: Token,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> __Symbol<>
    {
        match __token_index {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 13 | 14 | 15 | 16 | 17 | 18 | 20 | 21 | 22 | 23 | 26 | 27 | 28 => __Symbol::Variant0(__token),
            12 => match __token {
                Token::BoolConst(__tok0) if true => __Symbol::Variant1(__tok0),
                _ => unreachable!(),
            },
            19 | 24 | 25 => match __token {
                Token::IntConst(__tok0) | Token::StringConst(__tok0) | Token::TypeId(__tok0) if true => __Symbol::Variant2(__tok0),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    pub struct ProgramParser {
        _priv: (),
    }

    impl ProgramParser {
        pub fn new() -> ProgramParser {
            ProgramParser {
                _priv: (),
            }
        }

        #[allow(dead_code)]
        pub fn parse<
            'input,
            __TOKEN: __ToTriple<'input, >,
            __TOKENS: IntoIterator<Item=__TOKEN>,
        >(
            &self,
            __tokens0: __TOKENS,
        ) -> Result<alloc::vec::Vec<Expr>, __lalrpop_util::ParseError<(), Token, &'static str>>
        {
            let __tokens = __tokens0.into_iter();
            let mut __tokens = __tokens.map(|t| __ToTriple::to_triple(t));
            __state_machine::Parser::drive(
                __StateMachine {
                    __phantom: core::marker::PhantomData::<(&())>,
                },
                __tokens,
            )
        }
    }
    pub(crate) fn __reduce<
        'input,
    >(
        __action: i8,
        __lookahead_start: Option<&()>,
        __states: &mut alloc::vec::Vec<i8>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> Option<Result<alloc::vec::Vec<Expr>,__lalrpop_util::ParseError<(), Token, &'static str>>>
    {
        let (__pop_states, __nonterminal) = match __action {
            0 => {
                __reduce0(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            1 => {
                __reduce1(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            2 => {
                // __Program = Program => ActionFn(0);
                let __sym0 = __pop_Variant3(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(__sym0);
                return Some(Ok(__nt));
            }
            3 => {
                __reduce3(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            4 => {
                __reduce4(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            5 => {
                __reduce5(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            6 => {
                __reduce6(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            7 => {
                __reduce7(__lookahead_start, __symbols, core::marker::PhantomData::<(&())>)
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __states_len = __states.len();
        __states.truncate(__states_len - __pop_states);
        let __state = *__states.last().unwrap();
        let __next_state = __goto(__state, __nonterminal);
        __states.push(__next_state);
        None
    }
    #[inline(never)]
    fn __symbol_type_mismatch() -> ! {
        panic!("symbol type mismatch")
    }
    fn __pop_Variant4<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Expr, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant4(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant2<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), String, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant2(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant0<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), Token, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant0(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant3<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), alloc::vec::Vec<Expr>, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant3(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    fn __pop_Variant1<
    >(
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>
    ) -> ((), bool, ())
     {
        match __symbols.pop() {
            Some((__l, __Symbol::Variant1(__v), __r)) => (__l, __v, __r),
            _ => __symbol_type_mismatch()
        }
    }
    pub(crate) fn __reduce0<
        'input,
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program =  => ActionFn(7);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action7::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 0)
    }
    pub(crate) fn __reduce1<
        'input,
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // Program = atom+ => ActionFn(8);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action8::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 0)
    }
    pub(crate) fn __reduce3<
        'input,
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // atom = "Bool" => ActionFn(2);
        let __sym0 = __pop_Variant1(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action2::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant4(__nt), __end));
        (1, 2)
    }
    pub(crate) fn __reduce4<
        'input,
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // atom* =  => ActionFn(3);
        let __start = __lookahead_start.cloned().or_else(|| __symbols.last().map(|s| s.2.clone())).unwrap_or_default();
        let __end = __start.clone();
        let __nt = super::__action3::<>(&__start, &__end);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (0, 3)
    }
    pub(crate) fn __reduce5<
        'input,
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // atom* = atom+ => ActionFn(4);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action4::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 3)
    }
    pub(crate) fn __reduce6<
        'input,
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // atom+ = atom => ActionFn(5);
        let __sym0 = __pop_Variant4(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym0.2.clone();
        let __nt = super::__action5::<>(__sym0);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (1, 4)
    }
    pub(crate) fn __reduce7<
        'input,
    >(
        __lookahead_start: Option<&()>,
        __symbols: &mut alloc::vec::Vec<((),__Symbol<>,())>,
        _: core::marker::PhantomData<(&'input ())>,
    ) -> (usize, usize)
    {
        // atom+ = atom+, atom => ActionFn(6);
        assert!(__symbols.len() >= 2);
        let __sym1 = __pop_Variant4(__symbols);
        let __sym0 = __pop_Variant3(__symbols);
        let __start = __sym0.0.clone();
        let __end = __sym1.2.clone();
        let __nt = super::__action6::<>(__sym0, __sym1);
        __symbols.push((__start, __Symbol::Variant3(__nt), __end));
        (2, 4)
    }
}
pub use self::__parse__Program::ProgramParser;

fn __action0<
    'input,
>(
    (_, __0, _): ((), alloc::vec::Vec<Expr>, ()),
) -> alloc::vec::Vec<Expr>
{
    __0
}

fn __action1<
    'input,
>(
    (_, __0, _): ((), alloc::vec::Vec<Expr>, ()),
) -> alloc::vec::Vec<Expr>
{
    __0
}

fn __action2<
    'input,
>(
    (_, b, _): ((), bool, ()),
) -> Expr
{
    Expr::Bool(b)
}

fn __action3<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> alloc::vec::Vec<Expr>
{
    alloc::vec![]
}

fn __action4<
    'input,
>(
    (_, v, _): ((), alloc::vec::Vec<Expr>, ()),
) -> alloc::vec::Vec<Expr>
{
    v
}

fn __action5<
    'input,
>(
    (_, __0, _): ((), Expr, ()),
) -> alloc::vec::Vec<Expr>
{
    alloc::vec![__0]
}

fn __action6<
    'input,
>(
    (_, v, _): ((), alloc::vec::Vec<Expr>, ()),
    (_, e, _): ((), Expr, ()),
) -> alloc::vec::Vec<Expr>
{
    { let mut v = v; v.push(e); v }
}

fn __action7<
    'input,
>(
    __lookbehind: &(),
    __lookahead: &(),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action3(
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

fn __action8<
    'input,
>(
    __0: ((), alloc::vec::Vec<Expr>, ()),
) -> alloc::vec::Vec<Expr>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action4(
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action1(
        __temp0,
    )
}

pub trait __ToTriple<'input, >
{
    fn to_triple(value: Self) -> Result<((),Token,()), __lalrpop_util::ParseError<(), Token, &'static str>>;
}

impl<'input, > __ToTriple<'input, > for Token
{
    fn to_triple(value: Self) -> Result<((),Token,()), __lalrpop_util::ParseError<(), Token, &'static str>> {
        Ok(((), value, ()))
    }
}
impl<'input, > __ToTriple<'input, > for Result<Token,&'static str>
{
    fn to_triple(value: Self) -> Result<((),Token,()), __lalrpop_util::ParseError<(), Token, &'static str>> {
        match value {
            Ok(v) => Ok(((), v, ())),
            Err(error) => Err(__lalrpop_util::ParseError::User { error }),
        }
    }
}
