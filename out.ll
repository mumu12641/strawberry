; ModuleID = 'test.st'
source_filename = "test.st"

@"0_int_const" = protected global { i32, i64, i32, i32 } <{ i32 0, i64 114514, i32 1, i32 20 }>
@"1_int_const" = protected global { i32, i64, i32, i32 } <{ i32 1, i64 114514, i32 1, i32 20 }>
@"2_str_const_struct" = protected global { i64, i32, i64, i32, i32 } <{ i64 114514, i32 5, i64 114514, i32 2, i32 114514 }>

define i16 @main() {
main:
  ret void
}
