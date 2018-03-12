// auto-generated: "lalrpop 0.14.0"
use std::str::FromStr;
use ast;
#[allow(unused_extern_crates)]
extern crate lalrpop_util as __lalrpop_util;

mod __parse__Module {
    #![allow(non_snake_case, non_camel_case_types, unused_mut, unused_variables, unused_imports, unused_parens)]

    use std::str::FromStr;
    use ast;
    #[allow(unused_extern_crates)]
    extern crate lalrpop_util as __lalrpop_util;
    use super::__intern_token::Token;
    #[allow(dead_code)]
    pub enum __Symbol<'input>
     {
        Term_22_23_22(&'input str),
        Term_22_28_22(&'input str),
        Term_22_29_22(&'input str),
        Term_22_2c_22(&'input str),
        Term_22_3a_22(&'input str),
        Term_22_3b_22(&'input str),
        Term_22_3d_22(&'input str),
        Term_22_5b_22(&'input str),
        Term_22_5d_22(&'input str),
        Term_22bit_22(&'input str),
        Term_22byte_22(&'input str),
        Term_22inout_22(&'input str),
        Term_22input_22(&'input str),
        Term_22int_22(&'input str),
        Term_22integer_22(&'input str),
        Term_22logic_22(&'input str),
        Term_22longint_22(&'input str),
        Term_22module_22(&'input str),
        Term_22output_22(&'input str),
        Term_22parameter_22(&'input str),
        Term_22reg_22(&'input str),
        Term_22shortint_22(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_27b_5b01___5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_27d_5b0_2d9___5d_2b_22_23(&'input str),
        Termr_23_22_5b0_2d9_5d_2b_5c_27x_5b0_2d9a_2dfA_2dF___5d_2b_22_23(&'input str),
        Termr_23_22_5ba_2dzA_2dZ___5d_5b0_2d9A_2dZa_2dz___5d_2a_22_23(&'input str),
        Termr_23_22_5c_5cd_2b_22_23(&'input str),
        Nt_28_3cModuleArg_3e_20_22_2c_22_29(ast::Arg),
        Nt_28_3cModuleArg_3e_20_22_2c_22_29_2a(::std::vec::Vec<ast::Arg>),
        Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(::std::vec::Vec<ast::Arg>),
        Nt_28_3cParamArg_3e_20_22_2c_22_29(ast::ParamArg),
        Nt_28_3cParamArg_3e_20_22_2c_22_29_2a(::std::vec::Vec<ast::ParamArg>),
        Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(::std::vec::Vec<ast::ParamArg>),
        NtArgType(ast::Dir),
        NtBitRange(ast::BitRange),
        NtBitRange_3f(::std::option::Option<ast::BitRange>),
        NtComma_3cModuleArg_3e(Vec<ast::Arg>),
        NtComma_3cParamArg_3e(Vec<ast::ParamArg>),
        NtDataType(ast::DataType),
        NtDataType_3f(::std::option::Option<ast::DataType>),
        NtIdent(ast::Ident),
        NtModule(ast::Module),
        NtModuleArg(ast::Arg),
        NtModuleArg_3f(::std::option::Option<ast::Arg>),
        NtModuleArgs(Vec<ast::Arg>),
        NtNum(ast::Num),
        NtParamArg(ast::ParamArg),
        NtParamArg_3f(::std::option::Option<ast::ParamArg>),
        NtParamArgs(Vec<ast::ParamArg>),
        Nt____Module(ast::Module),
    }
    const __ACTION: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 3
        6, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        -35, -35, -35, -35, 0, 0, -35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, -20, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 22, 0, 23, 24, 0, 0, 25, 26, 27, 28, 0, 0, 0, 29, 30, 0, 0, 0, 5, 0,
        // State 10
        0, 0, -44, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, -17, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, -13, 0, -13, -13, 0, 0, -13, -13, -13, -13, 0, 0, 0, -13, -13, 0, 0, 0, -13, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, -12, 0, -12, -12, 0, 0, -12, -12, -12, -12, 0, 0, 0, -12, -12, 0, 0, 0, -12, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, -11, 0, -11, -11, 0, 0, -11, -11, -11, -11, 0, 0, 0, -11, -11, 0, 0, 0, -11, 0,
        // State 16
        0, 0, -22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 17
        0, 0, -19, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 20
        0, 0, -41, -41, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 46,
        // State 22
        0, 0, 0, 0, 0, 0, 0, -27, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -27, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, -29, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -29, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, -30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -30, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, -26, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -26, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, -28, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -28, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, -32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -32, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, -25, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -25, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, -31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -31, 0,
        // State 30
        0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0, -4, -4, 0, 0, 0, 0, 0, -4, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, -24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 33
        0, 0, -52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, -21, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, -39, -39, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 5, 0,
        // State 40
        0, 0, -40, -40, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 53, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, -47, -47, -47, 0, 0, 0, -47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, -46, -46, -46, 0, 0, 0, -46, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, -48, -48, -48, 0, 0, 0, -48, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, -45, -45, -45, 0, 0, 0, -45, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, -23, 54, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 46,
        // State 49
        0, 0, -9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -9, 0,
        // State 50
        0, 56, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, -38, -38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 43, 44, 45, 0, 46,
        // State 53
        0, 0, -10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -10, 0,
        // State 54
        0, 0, -49, -49, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, -18, 0, 0, 0, 0, 0, 0, 0, 0, 14, 15, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 59, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -14, 0,
        // State 59
        0, 0, 0, 0, 0, 61, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    const __EOF_ACTION: &'static [i32] = &[
        // State 0
        0,
        // State 1
        -53,
        // State 2
        0,
        // State 3
        0,
        // State 4
        0,
        // State 5
        0,
        // State 6
        0,
        // State 7
        0,
        // State 8
        0,
        // State 9
        0,
        // State 10
        0,
        // State 11
        0,
        // State 12
        0,
        // State 13
        0,
        // State 14
        0,
        // State 15
        0,
        // State 16
        0,
        // State 17
        0,
        // State 18
        0,
        // State 19
        0,
        // State 20
        0,
        // State 21
        0,
        // State 22
        0,
        // State 23
        0,
        // State 24
        0,
        // State 25
        0,
        // State 26
        0,
        // State 27
        0,
        // State 28
        0,
        // State 29
        0,
        // State 30
        0,
        // State 31
        0,
        // State 32
        0,
        // State 33
        0,
        // State 34
        0,
        // State 35
        0,
        // State 36
        0,
        // State 37
        0,
        // State 38
        0,
        // State 39
        0,
        // State 40
        0,
        // State 41
        0,
        // State 42
        0,
        // State 43
        0,
        // State 44
        0,
        // State 45
        0,
        // State 46
        -36,
        // State 47
        0,
        // State 48
        0,
        // State 49
        0,
        // State 50
        0,
        // State 51
        0,
        // State 52
        0,
        // State 53
        0,
        // State 54
        0,
        // State 55
        0,
        // State 56
        0,
        // State 57
        0,
        // State 58
        0,
        // State 59
        0,
        // State 60
        -37,
    ];
    const __GOTO: &'static [i32] = &[
        // State 0
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 1
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 2
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 3
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 4
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 5
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 6
        0, 0, 9, 0, 0, 0, 10, 0, 0, 11, 0, 0, 0, 0, 0, 12, 0, 13, 0, 0, 0, 0, 0,
        // State 7
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 8
        0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 18, 0, 0, 0, 0, 0, 0, 0,
        // State 9
        0, 0, 0, 0, 0, 0, 0, 19, 0, 0, 0, 20, 0, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 10
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 11
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 12
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 13
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 14
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 15
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 16
        0, 0, 0, 0, 0, 33, 0, 0, 0, 0, 34, 0, 0, 35, 0, 0, 0, 0, 0, 36, 0, 37, 0,
        // State 17
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 18
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 19
        0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 41, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 20
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 21
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 42, 0, 0, 0, 0,
        // State 22
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 23
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 24
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 25
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 26
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 27
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 28
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 29
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 30
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 31
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 32
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 0, 0, 0, 0, 0, 48, 0, 0, 0,
        // State 33
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 34
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 35
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 36
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 37
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 38
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 39
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 40
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 41
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 42
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 43
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 44
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 45
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 46
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 47
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 48
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0,
        // State 49
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 50
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 51
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 52
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 57, 0, 0, 0, 0,
        // State 53
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 54
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 55
        0, 0, 9, 0, 0, 0, 10, 0, 0, 11, 0, 0, 0, 0, 0, 12, 0, 58, 0, 0, 0, 0, 0,
        // State 56
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 57
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 58
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 59
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        // State 60
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];
    fn __expected_tokens(__state: usize) -> Vec<::std::string::String> {
        const __TERMINAL: &'static [&'static str] = &[
            r###""#""###,
            r###""(""###,
            r###"")""###,
            r###"",""###,
            r###"":""###,
            r###"";""###,
            r###""=""###,
            r###""[""###,
            r###""]""###,
            r###""bit""###,
            r###""byte""###,
            r###""inout""###,
            r###""input""###,
            r###""int""###,
            r###""integer""###,
            r###""logic""###,
            r###""longint""###,
            r###""module""###,
            r###""output""###,
            r###""parameter""###,
            r###""reg""###,
            r###""shortint""###,
            r###"r#"[0-9]+\'b[01_]+"#"###,
            r###"r#"[0-9]+\'d[0-9_]+"#"###,
            r###"r#"[0-9]+\'x[0-9a-fA-F_]+"#"###,
            r###"r#"[a-zA-Z_][0-9A-Za-z_]*"#"###,
            r###"r#"\\d+"#"###,
        ];
        __ACTION[(__state * 27)..].iter().zip(__TERMINAL).filter_map(|(&state, terminal)| {
            if state == 0 {
                None
            } else {
                Some(terminal.to_string())
            }
        }).collect()
    }
    #[allow(dead_code)]
    pub fn parse_Module<
        'input,
    >(
        input: &'input str,
    ) -> Result<ast::Module, __lalrpop_util::ParseError<usize, Token<'input>, &'static str>>
    {
        let mut __tokens = super::__intern_token::__Matcher::new(input);
        let mut __states = vec![0_i32];
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
                Token(5, _) if true => 0,
                Token(6, _) if true => 1,
                Token(7, _) if true => 2,
                Token(8, _) if true => 3,
                Token(9, _) if true => 4,
                Token(10, _) if true => 5,
                Token(11, _) if true => 6,
                Token(12, _) if true => 7,
                Token(13, _) if true => 8,
                Token(14, _) if true => 9,
                Token(15, _) if true => 10,
                Token(16, _) if true => 11,
                Token(17, _) if true => 12,
                Token(18, _) if true => 13,
                Token(19, _) if true => 14,
                Token(20, _) if true => 15,
                Token(21, _) if true => 16,
                Token(22, _) if true => 17,
                Token(23, _) if true => 18,
                Token(24, _) if true => 19,
                Token(25, _) if true => 20,
                Token(26, _) if true => 21,
                Token(0, _) if true => 22,
                Token(1, _) if true => 23,
                Token(2, _) if true => 24,
                Token(3, _) if true => 25,
                Token(4, _) if true => 26,
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
                let __action = __ACTION[__state * 27 + __integer];
                if __action > 0 {
                    let __symbol = match __integer {
                        0 => match __lookahead.1 {
                            Token(5, __tok0) => __Symbol::Term_22_23_22((__tok0)),
                            _ => unreachable!(),
                        },
                        1 => match __lookahead.1 {
                            Token(6, __tok0) => __Symbol::Term_22_28_22((__tok0)),
                            _ => unreachable!(),
                        },
                        2 => match __lookahead.1 {
                            Token(7, __tok0) => __Symbol::Term_22_29_22((__tok0)),
                            _ => unreachable!(),
                        },
                        3 => match __lookahead.1 {
                            Token(8, __tok0) => __Symbol::Term_22_2c_22((__tok0)),
                            _ => unreachable!(),
                        },
                        4 => match __lookahead.1 {
                            Token(9, __tok0) => __Symbol::Term_22_3a_22((__tok0)),
                            _ => unreachable!(),
                        },
                        5 => match __lookahead.1 {
                            Token(10, __tok0) => __Symbol::Term_22_3b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        6 => match __lookahead.1 {
                            Token(11, __tok0) => __Symbol::Term_22_3d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        7 => match __lookahead.1 {
                            Token(12, __tok0) => __Symbol::Term_22_5b_22((__tok0)),
                            _ => unreachable!(),
                        },
                        8 => match __lookahead.1 {
                            Token(13, __tok0) => __Symbol::Term_22_5d_22((__tok0)),
                            _ => unreachable!(),
                        },
                        9 => match __lookahead.1 {
                            Token(14, __tok0) => __Symbol::Term_22bit_22((__tok0)),
                            _ => unreachable!(),
                        },
                        10 => match __lookahead.1 {
                            Token(15, __tok0) => __Symbol::Term_22byte_22((__tok0)),
                            _ => unreachable!(),
                        },
                        11 => match __lookahead.1 {
                            Token(16, __tok0) => __Symbol::Term_22inout_22((__tok0)),
                            _ => unreachable!(),
                        },
                        12 => match __lookahead.1 {
                            Token(17, __tok0) => __Symbol::Term_22input_22((__tok0)),
                            _ => unreachable!(),
                        },
                        13 => match __lookahead.1 {
                            Token(18, __tok0) => __Symbol::Term_22int_22((__tok0)),
                            _ => unreachable!(),
                        },
                        14 => match __lookahead.1 {
                            Token(19, __tok0) => __Symbol::Term_22integer_22((__tok0)),
                            _ => unreachable!(),
                        },
                        15 => match __lookahead.1 {
                            Token(20, __tok0) => __Symbol::Term_22logic_22((__tok0)),
                            _ => unreachable!(),
                        },
                        16 => match __lookahead.1 {
                            Token(21, __tok0) => __Symbol::Term_22longint_22((__tok0)),
                            _ => unreachable!(),
                        },
                        17 => match __lookahead.1 {
                            Token(22, __tok0) => __Symbol::Term_22module_22((__tok0)),
                            _ => unreachable!(),
                        },
                        18 => match __lookahead.1 {
                            Token(23, __tok0) => __Symbol::Term_22output_22((__tok0)),
                            _ => unreachable!(),
                        },
                        19 => match __lookahead.1 {
                            Token(24, __tok0) => __Symbol::Term_22parameter_22((__tok0)),
                            _ => unreachable!(),
                        },
                        20 => match __lookahead.1 {
                            Token(25, __tok0) => __Symbol::Term_22reg_22((__tok0)),
                            _ => unreachable!(),
                        },
                        21 => match __lookahead.1 {
                            Token(26, __tok0) => __Symbol::Term_22shortint_22((__tok0)),
                            _ => unreachable!(),
                        },
                        22 => match __lookahead.1 {
                            Token(0, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_27b_5b01___5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        23 => match __lookahead.1 {
                            Token(1, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_27d_5b0_2d9___5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        24 => match __lookahead.1 {
                            Token(2, __tok0) => __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_27x_5b0_2d9a_2dfA_2dF___5d_2b_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        25 => match __lookahead.1 {
                            Token(3, __tok0) => __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5b0_2d9A_2dZa_2dz___5d_2a_22_23((__tok0)),
                            _ => unreachable!(),
                        },
                        26 => match __lookahead.1 {
                            Token(4, __tok0) => __Symbol::Termr_23_22_5c_5cd_2b_22_23((__tok0)),
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
    pub fn __reduce<
        'input,
    >(
        input: &'input str,
        __action: i32,
        __lookahead_start: Option<&usize>,
        __states: &mut ::std::vec::Vec<i32>,
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>,
        _: ::std::marker::PhantomData<()>,
    ) -> Option<Result<ast::Module,__lalrpop_util::ParseError<usize, Token<'input>, &'static str>>>
    {
        let __nonterminal = match -__action {
            1 => {
                // (<ModuleArg> ",") = ModuleArg, "," => ActionFn(39);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtModuleArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action39::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29(__nt), __end));
                0
            }
            2 => {
                // (<ModuleArg> ",")* =  => ActionFn(37);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action37::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            3 => {
                // (<ModuleArg> ",")* = (<ModuleArg> ",")+ => ActionFn(38);
                let __sym0 = __pop_Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action38::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29_2a(__nt), __end));
                1
            }
            4 => {
                // (<ModuleArg> ",")+ = ModuleArg, "," => ActionFn(44);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtModuleArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action44::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            5 => {
                // (<ModuleArg> ",")+ = (<ModuleArg> ",")+, ModuleArg, "," => ActionFn(45);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtModuleArg(__symbols);
                let __sym0 = __pop_Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action45::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(__nt), __end));
                2
            }
            6 => {
                // (<ParamArg> ",") = ParamArg, "," => ActionFn(34);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtParamArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action34::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29(__nt), __end));
                3
            }
            7 => {
                // (<ParamArg> ",")* =  => ActionFn(32);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action32::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            8 => {
                // (<ParamArg> ",")* = (<ParamArg> ",")+ => ActionFn(33);
                let __sym0 = __pop_Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action33::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29_2a(__nt), __end));
                4
            }
            9 => {
                // (<ParamArg> ",")+ = ParamArg, "," => ActionFn(48);
                let __sym1 = __pop_Term_22_2c_22(__symbols);
                let __sym0 = __pop_NtParamArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action48::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            10 => {
                // (<ParamArg> ",")+ = (<ParamArg> ",")+, ParamArg, "," => ActionFn(49);
                let __sym2 = __pop_Term_22_2c_22(__symbols);
                let __sym1 = __pop_NtParamArg(__symbols);
                let __sym0 = __pop_Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action49::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(__nt), __end));
                5
            }
            11 => {
                // ArgType = "output" => ActionFn(8);
                let __sym0 = __pop_Term_22output_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action8::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtArgType(__nt), __end));
                6
            }
            12 => {
                // ArgType = "input" => ActionFn(9);
                let __sym0 = __pop_Term_22input_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action9::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtArgType(__nt), __end));
                6
            }
            13 => {
                // ArgType = "inout" => ActionFn(10);
                let __sym0 = __pop_Term_22inout_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action10::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtArgType(__nt), __end));
                6
            }
            14 => {
                // BitRange = "[", Num, ":", Num, "]" => ActionFn(7);
                let __sym4 = __pop_Term_22_5d_22(__symbols);
                let __sym3 = __pop_NtNum(__symbols);
                let __sym2 = __pop_Term_22_3a_22(__symbols);
                let __sym1 = __pop_NtNum(__symbols);
                let __sym0 = __pop_Term_22_5b_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym4.2.clone();
                let __nt = super::__action7::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4);
                let __states_len = __states.len();
                __states.truncate(__states_len - 5);
                __symbols.push((__start, __Symbol::NtBitRange(__nt), __end));
                7
            }
            15 => {
                // BitRange? = BitRange => ActionFn(24);
                let __sym0 = __pop_NtBitRange(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action24::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtBitRange_3f(__nt), __end));
                8
            }
            16 => {
                // BitRange? =  => ActionFn(25);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action25::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtBitRange_3f(__nt), __end));
                8
            }
            17 => {
                // Comma<ModuleArg> = ModuleArg => ActionFn(58);
                let __sym0 = __pop_NtModuleArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action58::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cModuleArg_3e(__nt), __end));
                9
            }
            18 => {
                // Comma<ModuleArg> =  => ActionFn(59);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action59::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cModuleArg_3e(__nt), __end));
                9
            }
            19 => {
                // Comma<ModuleArg> = (<ModuleArg> ",")+, ModuleArg => ActionFn(60);
                let __sym1 = __pop_NtModuleArg(__symbols);
                let __sym0 = __pop_Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action60::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cModuleArg_3e(__nt), __end));
                9
            }
            20 => {
                // Comma<ModuleArg> = (<ModuleArg> ",")+ => ActionFn(61);
                let __sym0 = __pop_Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action61::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cModuleArg_3e(__nt), __end));
                9
            }
            21 => {
                // Comma<ParamArg> = ParamArg => ActionFn(62);
                let __sym0 = __pop_NtParamArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action62::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cParamArg_3e(__nt), __end));
                10
            }
            22 => {
                // Comma<ParamArg> =  => ActionFn(63);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action63::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtComma_3cParamArg_3e(__nt), __end));
                10
            }
            23 => {
                // Comma<ParamArg> = (<ParamArg> ",")+, ParamArg => ActionFn(64);
                let __sym1 = __pop_NtParamArg(__symbols);
                let __sym0 = __pop_Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action64::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtComma_3cParamArg_3e(__nt), __end));
                10
            }
            24 => {
                // Comma<ParamArg> = (<ParamArg> ",")+ => ActionFn(65);
                let __sym0 = __pop_Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action65::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtComma_3cParamArg_3e(__nt), __end));
                10
            }
            25 => {
                // DataType = "reg" => ActionFn(11);
                let __sym0 = __pop_Term_22reg_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action11::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            26 => {
                // DataType = "integer" => ActionFn(12);
                let __sym0 = __pop_Term_22integer_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action12::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            27 => {
                // DataType = "bit" => ActionFn(13);
                let __sym0 = __pop_Term_22bit_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action13::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            28 => {
                // DataType = "logic" => ActionFn(14);
                let __sym0 = __pop_Term_22logic_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action14::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            29 => {
                // DataType = "byte" => ActionFn(15);
                let __sym0 = __pop_Term_22byte_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action15::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            30 => {
                // DataType = "int" => ActionFn(16);
                let __sym0 = __pop_Term_22int_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action16::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            31 => {
                // DataType = "shortint" => ActionFn(17);
                let __sym0 = __pop_Term_22shortint_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action17::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            32 => {
                // DataType = "longint" => ActionFn(18);
                let __sym0 = __pop_Term_22longint_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action18::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType(__nt), __end));
                11
            }
            33 => {
                // DataType? = DataType => ActionFn(26);
                let __sym0 = __pop_NtDataType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action26::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtDataType_3f(__nt), __end));
                12
            }
            34 => {
                // DataType? =  => ActionFn(27);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action27::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtDataType_3f(__nt), __end));
                12
            }
            35 => {
                // Ident = r#"[a-zA-Z_][0-9A-Za-z_]*"# => ActionFn(19);
                let __sym0 = __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5b0_2d9A_2dZa_2dz___5d_2a_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action19::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtIdent(__nt), __end));
                13
            }
            36 => {
                // Module = "module", Ident, "(", ModuleArgs, ")", ";" => ActionFn(1);
                let __sym5 = __pop_Term_22_3b_22(__symbols);
                let __sym4 = __pop_Term_22_29_22(__symbols);
                let __sym3 = __pop_NtModuleArgs(__symbols);
                let __sym2 = __pop_Term_22_28_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22module_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym5.2.clone();
                let __nt = super::__action1::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5);
                let __states_len = __states.len();
                __states.truncate(__states_len - 6);
                __symbols.push((__start, __Symbol::NtModule(__nt), __end));
                14
            }
            37 => {
                // Module = "module", Ident, "#", "(", "parameter", ParamArgs, ")", "(", ModuleArgs, ")", ";" => ActionFn(2);
                let __sym10 = __pop_Term_22_3b_22(__symbols);
                let __sym9 = __pop_Term_22_29_22(__symbols);
                let __sym8 = __pop_NtModuleArgs(__symbols);
                let __sym7 = __pop_Term_22_28_22(__symbols);
                let __sym6 = __pop_Term_22_29_22(__symbols);
                let __sym5 = __pop_NtParamArgs(__symbols);
                let __sym4 = __pop_Term_22parameter_22(__symbols);
                let __sym3 = __pop_Term_22_28_22(__symbols);
                let __sym2 = __pop_Term_22_23_22(__symbols);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_Term_22module_22(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym10.2.clone();
                let __nt = super::__action2::<>(input, __sym0, __sym1, __sym2, __sym3, __sym4, __sym5, __sym6, __sym7, __sym8, __sym9, __sym10);
                let __states_len = __states.len();
                __states.truncate(__states_len - 11);
                __symbols.push((__start, __Symbol::NtModule(__nt), __end));
                14
            }
            38 => {
                // ModuleArg = ArgType, DataType, BitRange, Ident => ActionFn(54);
                let __sym3 = __pop_NtIdent(__symbols);
                let __sym2 = __pop_NtBitRange(__symbols);
                let __sym1 = __pop_NtDataType(__symbols);
                let __sym0 = __pop_NtArgType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym3.2.clone();
                let __nt = super::__action54::<>(input, __sym0, __sym1, __sym2, __sym3);
                let __states_len = __states.len();
                __states.truncate(__states_len - 4);
                __symbols.push((__start, __Symbol::NtModuleArg(__nt), __end));
                15
            }
            39 => {
                // ModuleArg = ArgType, BitRange, Ident => ActionFn(55);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_NtBitRange(__symbols);
                let __sym0 = __pop_NtArgType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action55::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtModuleArg(__nt), __end));
                15
            }
            40 => {
                // ModuleArg = ArgType, DataType, Ident => ActionFn(56);
                let __sym2 = __pop_NtIdent(__symbols);
                let __sym1 = __pop_NtDataType(__symbols);
                let __sym0 = __pop_NtArgType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action56::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtModuleArg(__nt), __end));
                15
            }
            41 => {
                // ModuleArg = ArgType, Ident => ActionFn(57);
                let __sym1 = __pop_NtIdent(__symbols);
                let __sym0 = __pop_NtArgType(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym1.2.clone();
                let __nt = super::__action57::<>(input, __sym0, __sym1);
                let __states_len = __states.len();
                __states.truncate(__states_len - 2);
                __symbols.push((__start, __Symbol::NtModuleArg(__nt), __end));
                15
            }
            42 => {
                // ModuleArg? = ModuleArg => ActionFn(35);
                let __sym0 = __pop_NtModuleArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action35::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtModuleArg_3f(__nt), __end));
                16
            }
            43 => {
                // ModuleArg? =  => ActionFn(36);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action36::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtModuleArg_3f(__nt), __end));
                16
            }
            44 => {
                // ModuleArgs = Comma<ModuleArg> => ActionFn(5);
                let __sym0 = __pop_NtComma_3cModuleArg_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action5::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtModuleArgs(__nt), __end));
                17
            }
            45 => {
                // Num = r#"\\d+"# => ActionFn(20);
                let __sym0 = __pop_Termr_23_22_5c_5cd_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action20::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                18
            }
            46 => {
                // Num = r#"[0-9]+\'d[0-9_]+"# => ActionFn(21);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_27d_5b0_2d9___5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action21::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                18
            }
            47 => {
                // Num = r#"[0-9]+\'b[01_]+"# => ActionFn(22);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_27b_5b01___5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action22::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                18
            }
            48 => {
                // Num = r#"[0-9]+\'x[0-9a-fA-F_]+"# => ActionFn(23);
                let __sym0 = __pop_Termr_23_22_5b0_2d9_5d_2b_5c_27x_5b0_2d9a_2dfA_2dF___5d_2b_22_23(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action23::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtNum(__nt), __end));
                18
            }
            49 => {
                // ParamArg = Ident, "=", Num => ActionFn(4);
                let __sym2 = __pop_NtNum(__symbols);
                let __sym1 = __pop_Term_22_3d_22(__symbols);
                let __sym0 = __pop_NtIdent(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym2.2.clone();
                let __nt = super::__action4::<>(input, __sym0, __sym1, __sym2);
                let __states_len = __states.len();
                __states.truncate(__states_len - 3);
                __symbols.push((__start, __Symbol::NtParamArg(__nt), __end));
                19
            }
            50 => {
                // ParamArg? = ParamArg => ActionFn(30);
                let __sym0 = __pop_NtParamArg(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action30::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtParamArg_3f(__nt), __end));
                20
            }
            51 => {
                // ParamArg? =  => ActionFn(31);
                let __start = __symbols.last().map(|s| s.2.clone()).unwrap_or_default();
                let __end = __lookahead_start.cloned().unwrap_or_else(|| __start.clone());
                let __nt = super::__action31::<>(input, &__start, &__end);
                let __states_len = __states.len();
                __states.truncate(__states_len - 0);
                __symbols.push((__start, __Symbol::NtParamArg_3f(__nt), __end));
                20
            }
            52 => {
                // ParamArgs = Comma<ParamArg> => ActionFn(3);
                let __sym0 = __pop_NtComma_3cParamArg_3e(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action3::<>(input, __sym0);
                let __states_len = __states.len();
                __states.truncate(__states_len - 1);
                __symbols.push((__start, __Symbol::NtParamArgs(__nt), __end));
                21
            }
            53 => {
                // __Module = Module => ActionFn(0);
                let __sym0 = __pop_NtModule(__symbols);
                let __start = __sym0.0.clone();
                let __end = __sym0.2.clone();
                let __nt = super::__action0::<>(input, __sym0);
                return Some(Ok(__nt));
            }
            _ => panic!("invalid action code {}", __action)
        };
        let __state = *__states.last().unwrap() as usize;
        let __next_state = __GOTO[__state * 23 + __nonterminal] - 1;
        __states.push(__next_state);
        None
    }
    fn __pop_Term_22_23_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_23_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_28_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_28_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_29_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_29_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_2c_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_2c_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3a_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3a_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_3d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_3d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5b_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5b_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22_5d_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22_5d_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22bit_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22bit_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22byte_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22byte_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22inout_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22inout_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22input_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22input_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22int_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22int_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22integer_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22integer_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22logic_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22logic_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22longint_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22longint_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22module_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22module_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22output_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22output_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22parameter_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22parameter_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22reg_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22reg_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Term_22shortint_22<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Term_22shortint_22(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_27b_5b01___5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_27b_5b01___5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_27d_5b0_2d9___5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_27d_5b0_2d9___5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5b0_2d9_5d_2b_5c_27x_5b0_2d9a_2dfA_2dF___5d_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5b0_2d9_5d_2b_5c_27x_5b0_2d9a_2dfA_2dF___5d_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5ba_2dzA_2dZ___5d_5b0_2d9A_2dZa_2dz___5d_2a_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5ba_2dzA_2dZ___5d_5b0_2d9A_2dZa_2dz___5d_2a_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Termr_23_22_5c_5cd_2b_22_23<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, &'input str, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Termr_23_22_5c_5cd_2b_22_23(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cModuleArg_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Arg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cModuleArg_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Arg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::Arg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cModuleArg_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cParamArg_3e_20_22_2c_22_29<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::ParamArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cParamArg_3e_20_22_2c_22_29_2a<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ParamArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29_2a(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt_28_3cParamArg_3e_20_22_2c_22_29_2b<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::vec::Vec<ast::ParamArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt_28_3cParamArg_3e_20_22_2c_22_29_2b(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtArgType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Dir, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtArgType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitRange<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::BitRange, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitRange(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtBitRange_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<ast::BitRange>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtBitRange_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cModuleArg_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ast::Arg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cModuleArg_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtComma_3cParamArg_3e<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ast::ParamArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtComma_3cParamArg_3e(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDataType<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::DataType, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDataType(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtDataType_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<ast::DataType>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtDataType_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtIdent<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Ident, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtIdent(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtModule<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Module, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtModule(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtModuleArg<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Arg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtModuleArg(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtModuleArg_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<ast::Arg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtModuleArg_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtModuleArgs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ast::Arg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtModuleArgs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtNum<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Num, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtNum(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtParamArg<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::ParamArg, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParamArg(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtParamArg_3f<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ::std::option::Option<ast::ParamArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParamArg_3f(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_NtParamArgs<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, Vec<ast::ParamArg>, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::NtParamArgs(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
    fn __pop_Nt____Module<
      'input,
    >(
        __symbols: &mut ::std::vec::Vec<(usize,__Symbol<'input>,usize)>
    ) -> (usize, ast::Module, usize)
     {
        match __symbols.pop().unwrap() {
            (__l, __Symbol::Nt____Module(__v), __r) => (__l, __v, __r),
            _ => panic!("symbol type mismatch")
        }
    }
}
pub use self::__parse__Module::parse_Module;
mod __intern_token {
    #![allow(unused_imports)]
    use std::str::FromStr;
    use ast;
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

    pub struct __Matcher<'input> {
        text: &'input str,
        consumed: usize,
        regex_set: __regex::RegexSet,
        regex_vec: Vec<__regex::Regex>,
    }

    impl<'input> __Matcher<'input> {
        pub fn new(s: &'input str) -> __Matcher<'input> {
            let __strs: &[&str] = &[
                "^(?u:[0-9])+(?u:\'b)(?u:[0-1_-_])+",
                "^(?u:[0-9])+(?u:\'d)(?u:[0-9_-_])+",
                "^(?u:[0-9])+(?u:\'x)(?u:[0-9A-F_-_a-f])+",
                "^(?u:[A-Z_-_a-z])(?u:[0-9A-Z_-_a-z])*",
                "^(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙])+",
                "^(?u:\\#)",
                "^(?u:\\()",
                "^(?u:\\))",
                "^(?u:,)",
                "^(?u::)",
                "^(?u:;)",
                "^(?u:=)",
                "^(?u:\\[)",
                "^(?u:\\])",
                "^(?u:bit)",
                "^(?u:byte)",
                "^(?u:inout)",
                "^(?u:input)",
                "^(?u:int)",
                "^(?u:integer)",
                "^(?u:logic)",
                "^(?u:longint)",
                "^(?u:module)",
                "^(?u:output)",
                "^(?u:parameter)",
                "^(?u:reg)",
                "^(?u:shortint)",
            ];
            let __regex_set = __regex::RegexSet::new(__strs).unwrap();
            let __regex_vec = vec![
                __regex::Regex::new("^(?u:[0-9])+(?u:\'b)(?u:[0-1_-_])+").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+(?u:\'d)(?u:[0-9_-_])+").unwrap(),
                __regex::Regex::new("^(?u:[0-9])+(?u:\'x)(?u:[0-9A-F_-_a-f])+").unwrap(),
                __regex::Regex::new("^(?u:[A-Z_-_a-z])(?u:[0-9A-Z_-_a-z])*").unwrap(),
                __regex::Regex::new("^(?u:[0-9٠-٩۰-۹߀-߉०-९০-৯੦-੯૦-૯୦-୯௦-௯౦-౯೦-೯൦-൯෦-෯๐-๙໐-໙༠-༩၀-၉႐-႙០-៩᠐-᠙᥆-᥏᧐-᧙᪀-᪉᪐-᪙᭐-᭙᮰-᮹᱀-᱉᱐-᱙꘠-꘩꣐-꣙꤀-꤉꧐-꧙꧰-꧹꩐-꩙꯰-꯹０-９𐒠-𐒩𑁦-𑁯𑃰-𑃹𑄶-𑄿𑇐-𑇙𑋰-𑋹𑑐-𑑙𑓐-𑓙𑙐-𑙙𑛀-𑛉𑜰-𑜹𑣠-𑣩𑱐-𑱙𑵐-𑵙𖩠-𖩩𖭐-𖭙𝟎-𝟿𞥐-𞥙])+").unwrap(),
                __regex::Regex::new("^(?u:\\#)").unwrap(),
                __regex::Regex::new("^(?u:\\()").unwrap(),
                __regex::Regex::new("^(?u:\\))").unwrap(),
                __regex::Regex::new("^(?u:,)").unwrap(),
                __regex::Regex::new("^(?u::)").unwrap(),
                __regex::Regex::new("^(?u:;)").unwrap(),
                __regex::Regex::new("^(?u:=)").unwrap(),
                __regex::Regex::new("^(?u:\\[)").unwrap(),
                __regex::Regex::new("^(?u:\\])").unwrap(),
                __regex::Regex::new("^(?u:bit)").unwrap(),
                __regex::Regex::new("^(?u:byte)").unwrap(),
                __regex::Regex::new("^(?u:inout)").unwrap(),
                __regex::Regex::new("^(?u:input)").unwrap(),
                __regex::Regex::new("^(?u:int)").unwrap(),
                __regex::Regex::new("^(?u:integer)").unwrap(),
                __regex::Regex::new("^(?u:logic)").unwrap(),
                __regex::Regex::new("^(?u:longint)").unwrap(),
                __regex::Regex::new("^(?u:module)").unwrap(),
                __regex::Regex::new("^(?u:output)").unwrap(),
                __regex::Regex::new("^(?u:parameter)").unwrap(),
                __regex::Regex::new("^(?u:reg)").unwrap(),
                __regex::Regex::new("^(?u:shortint)").unwrap(),
            ];
            __Matcher {
                text: s,
                consumed: 0,
                regex_set: __regex_set,
                regex_vec: __regex_vec,
            }
        }
    }

    impl<'input> Iterator for __Matcher<'input> {
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
                    for __i in 0 .. 27 {
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
    (_, __0, _): (usize, ast::Module, usize),
) -> ast::Module
{
    (__0)
}

#[allow(unused_variables)]
fn __action1<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<ast::Arg>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::Module
{
    { ast::Module(i, None, a) }
}

#[allow(unused_variables)]
fn __action2<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, i, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, p, _): (usize, Vec<ast::ParamArg>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, a, _): (usize, Vec<ast::Arg>, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::Module
{
    { ast::Module(i, Some(p), a) }
}

#[allow(unused_variables)]
fn __action3<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<ast::ParamArg>, usize),
) -> Vec<ast::ParamArg>
{
    (__0)
}

#[allow(unused_variables)]
fn __action4<
    'input,
>(
    input: &'input str,
    (_, i, _): (usize, ast::Ident, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, n, _): (usize, ast::Num, usize),
) -> ast::ParamArg
{
    (i, n)
}

#[allow(unused_variables)]
fn __action5<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, Vec<ast::Arg>, usize),
) -> Vec<ast::Arg>
{
    (__0)
}

#[allow(unused_variables)]
fn __action6<
    'input,
>(
    input: &'input str,
    (_, d, _): (usize, ast::Dir, usize),
    (_, t, _): (usize, ::std::option::Option<ast::DataType>, usize),
    (_, b, _): (usize, ::std::option::Option<ast::BitRange>, usize),
    (_, i, _): (usize, ast::Ident, usize),
) -> ast::Arg
{
    (i, d, b, t)
}

#[allow(unused_variables)]
fn __action7<
    'input,
>(
    input: &'input str,
    (_, _, _): (usize, &'input str, usize),
    (_, __0, _): (usize, ast::Num, usize),
    (_, _, _): (usize, &'input str, usize),
    (_, __1, _): (usize, ast::Num, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::BitRange
{
    (__0, __1)
}

#[allow(unused_variables)]
fn __action8<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Dir
{
    ast::Dir::Output
}

#[allow(unused_variables)]
fn __action9<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Dir
{
    ast::Dir::Input
}

#[allow(unused_variables)]
fn __action10<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Dir
{
    ast::Dir::InOut
}

#[allow(unused_variables)]
fn __action11<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::Reg
}

#[allow(unused_variables)]
fn __action12<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::Integer
}

#[allow(unused_variables)]
fn __action13<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::Bit
}

#[allow(unused_variables)]
fn __action14<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::Logic
}

#[allow(unused_variables)]
fn __action15<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::Byte
}

#[allow(unused_variables)]
fn __action16<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::Int
}

#[allow(unused_variables)]
fn __action17<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::ShortInt
}

#[allow(unused_variables)]
fn __action18<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::DataType
{
    ast::DataType::LongInt
}

#[allow(unused_variables)]
fn __action19<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Ident
{
    ast::Ident(__0.to_string())
}

#[allow(unused_variables)]
fn __action20<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Num
{
    {
        let num = __0.to_string().replace("_", "");
        ast::Num(i32::from_str(__0).unwrap())
    }
}

#[allow(unused_variables)]
fn __action21<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Num
{
    {
        let num = __0.to_string().replace("_", "").splitn(2, 'd').nth(1).unwrap().to_string();
        ast::Num(i32::from_str_radix(&num, 10).unwrap())
    }
}

#[allow(unused_variables)]
fn __action22<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Num
{
    {
        let num = __0.to_string().replace("_", "").splitn(2, 'b').nth(1).unwrap().to_string();
        ast::Num(i32::from_str_radix(&num, 2).unwrap())
    }
}

#[allow(unused_variables)]
fn __action23<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, &'input str, usize),
) -> ast::Num
{
    {
        let num = __0.to_string().replace("_", "").splitn(2, 'x').nth(1).unwrap().to_string();
        ast::Num(i32::from_str_radix(&num, 16).unwrap())
    }
}

#[allow(unused_variables)]
fn __action24<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::BitRange, usize),
) -> ::std::option::Option<ast::BitRange>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action25<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<ast::BitRange>
{
    None
}

#[allow(unused_variables)]
fn __action26<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::DataType, usize),
) -> ::std::option::Option<ast::DataType>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action27<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<ast::DataType>
{
    None
}

#[allow(unused_variables)]
fn __action28<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ast::Arg>, usize),
    (_, e, _): (usize, ::std::option::Option<ast::Arg>, usize),
) -> Vec<ast::Arg>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action29<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ast::ParamArg>, usize),
    (_, e, _): (usize, ::std::option::Option<ast::ParamArg>, usize),
) -> Vec<ast::ParamArg>
{
    match e {
        None => v,
        Some(e) => {
            let mut v = v;
            v.push(e);
            v
        }
    }
}

#[allow(unused_variables)]
fn __action30<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::ParamArg, usize),
) -> ::std::option::Option<ast::ParamArg>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action31<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<ast::ParamArg>
{
    None
}

#[allow(unused_variables)]
fn __action32<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ast::ParamArg>
{
    vec![]
}

#[allow(unused_variables)]
fn __action33<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ast::ParamArg>, usize),
) -> ::std::vec::Vec<ast::ParamArg>
{
    v
}

#[allow(unused_variables)]
fn __action34<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::ParamArg, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::ParamArg
{
    (__0)
}

#[allow(unused_variables)]
fn __action35<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Arg, usize),
) -> ::std::option::Option<ast::Arg>
{
    Some(__0)
}

#[allow(unused_variables)]
fn __action36<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::option::Option<ast::Arg>
{
    None
}

#[allow(unused_variables)]
fn __action37<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> ::std::vec::Vec<ast::Arg>
{
    vec![]
}

#[allow(unused_variables)]
fn __action38<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ast::Arg>, usize),
) -> ::std::vec::Vec<ast::Arg>
{
    v
}

#[allow(unused_variables)]
fn __action39<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Arg, usize),
    (_, _, _): (usize, &'input str, usize),
) -> ast::Arg
{
    (__0)
}

#[allow(unused_variables)]
fn __action40<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::Arg, usize),
) -> ::std::vec::Vec<ast::Arg>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action41<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ast::Arg>, usize),
    (_, e, _): (usize, ast::Arg, usize),
) -> ::std::vec::Vec<ast::Arg>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action42<
    'input,
>(
    input: &'input str,
    (_, __0, _): (usize, ast::ParamArg, usize),
) -> ::std::vec::Vec<ast::ParamArg>
{
    vec![__0]
}

#[allow(unused_variables)]
fn __action43<
    'input,
>(
    input: &'input str,
    (_, v, _): (usize, ::std::vec::Vec<ast::ParamArg>, usize),
    (_, e, _): (usize, ast::ParamArg, usize),
) -> ::std::vec::Vec<ast::ParamArg>
{
    { let mut v = v; v.push(e); v }
}

#[allow(unused_variables)]
fn __action44<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Arg, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<ast::Arg>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action39(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action40(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action45<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::Arg>, usize),
    __1: (usize, ast::Arg, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<ast::Arg>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action39(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action41(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action46<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<ast::Arg>, usize),
) -> Vec<ast::Arg>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action37(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action47<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::Arg>, usize),
    __1: (usize, ::std::option::Option<ast::Arg>, usize),
) -> Vec<ast::Arg>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action38(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action28(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action48<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::ParamArg, usize),
    __1: (usize, &'input str, usize),
) -> ::std::vec::Vec<ast::ParamArg>
{
    let __start0 = __0.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action34(
        input,
        __0,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action42(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action49<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::ParamArg>, usize),
    __1: (usize, ast::ParamArg, usize),
    __2: (usize, &'input str, usize),
) -> ::std::vec::Vec<ast::ParamArg>
{
    let __start0 = __1.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action34(
        input,
        __1,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action43(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action50<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::option::Option<ast::ParamArg>, usize),
) -> Vec<ast::ParamArg>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.0.clone();
    let __temp0 = __action32(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __temp0,
        __0,
    )
}

#[allow(unused_variables)]
fn __action51<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::ParamArg>, usize),
    __1: (usize, ::std::option::Option<ast::ParamArg>, usize),
) -> Vec<ast::ParamArg>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action33(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action29(
        input,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action52<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Dir, usize),
    __1: (usize, ::std::option::Option<ast::DataType>, usize),
    __2: (usize, ast::BitRange, usize),
    __3: (usize, ast::Ident, usize),
) -> ast::Arg
{
    let __start0 = __2.0.clone();
    let __end0 = __2.2.clone();
    let __temp0 = __action24(
        input,
        __2,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __1,
        __temp0,
        __3,
    )
}

#[allow(unused_variables)]
fn __action53<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Dir, usize),
    __1: (usize, ::std::option::Option<ast::DataType>, usize),
    __2: (usize, ast::Ident, usize),
) -> ast::Arg
{
    let __start0 = __1.2.clone();
    let __end0 = __2.0.clone();
    let __temp0 = __action25(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action6(
        input,
        __0,
        __1,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action54<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Dir, usize),
    __1: (usize, ast::DataType, usize),
    __2: (usize, ast::BitRange, usize),
    __3: (usize, ast::Ident, usize),
) -> ast::Arg
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action26(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __0,
        __temp0,
        __2,
        __3,
    )
}

#[allow(unused_variables)]
fn __action55<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Dir, usize),
    __1: (usize, ast::BitRange, usize),
    __2: (usize, ast::Ident, usize),
) -> ast::Arg
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action52(
        input,
        __0,
        __temp0,
        __1,
        __2,
    )
}

#[allow(unused_variables)]
fn __action56<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Dir, usize),
    __1: (usize, ast::DataType, usize),
    __2: (usize, ast::Ident, usize),
) -> ast::Arg
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action26(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __0,
        __temp0,
        __2,
    )
}

#[allow(unused_variables)]
fn __action57<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Dir, usize),
    __1: (usize, ast::Ident, usize),
) -> ast::Arg
{
    let __start0 = __0.2.clone();
    let __end0 = __1.0.clone();
    let __temp0 = __action27(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action53(
        input,
        __0,
        __temp0,
        __1,
    )
}

#[allow(unused_variables)]
fn __action58<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::Arg, usize),
) -> Vec<ast::Arg>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action35(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action59<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::Arg>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action36(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action46(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action60<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::Arg>, usize),
    __1: (usize, ast::Arg, usize),
) -> Vec<ast::Arg>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action35(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action61<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::Arg>, usize),
) -> Vec<ast::Arg>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action36(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action47(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action62<
    'input,
>(
    input: &'input str,
    __0: (usize, ast::ParamArg, usize),
) -> Vec<ast::ParamArg>
{
    let __start0 = __0.0.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action30(
        input,
        __0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action63<
    'input,
>(
    input: &'input str,
    __lookbehind: &usize,
    __lookahead: &usize,
) -> Vec<ast::ParamArg>
{
    let __start0 = __lookbehind.clone();
    let __end0 = __lookahead.clone();
    let __temp0 = __action31(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action50(
        input,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action64<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::ParamArg>, usize),
    __1: (usize, ast::ParamArg, usize),
) -> Vec<ast::ParamArg>
{
    let __start0 = __1.0.clone();
    let __end0 = __1.2.clone();
    let __temp0 = __action30(
        input,
        __1,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __0,
        __temp0,
    )
}

#[allow(unused_variables)]
fn __action65<
    'input,
>(
    input: &'input str,
    __0: (usize, ::std::vec::Vec<ast::ParamArg>, usize),
) -> Vec<ast::ParamArg>
{
    let __start0 = __0.2.clone();
    let __end0 = __0.2.clone();
    let __temp0 = __action31(
        input,
        &__start0,
        &__end0,
    );
    let __temp0 = (__start0, __temp0, __end0);
    __action51(
        input,
        __0,
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
