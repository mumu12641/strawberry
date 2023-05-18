; ModuleID = 'test.st'
source_filename = "test.st"

@"0_int_const" = protected global { i32, i64, i32, i32 } <{ i32 0, i64 114514, i32 1, i32 20 }>
@"1_int_const" = protected global { i32, i64, i32, i32 } <{ i32 1, i64 114514, i32 1, i32 20 }>
@"0_str_const" = private unnamed_addr constant [6 x i8] c"hello\00", align 1
@"2_str_const_struct" = protected global { i64, i32, i64, i32, i32 } <{ i64 114514, i32 5, i64 114514, i32 2 }>
@"1_str_const" = private unnamed_addr constant [12 x i8] c"hello world\00", align 1
@"2_str_const_struct.1" = protected global { i64, i32, i64, i32, i32 } <{ i64 114514, i32 11, i64 114514, i32 2 }>

define i16 @main() {
main:
  ret void
}
