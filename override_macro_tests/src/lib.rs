mod t01_override_for_struct;
mod t02_skip_trait_function;
mod t03_skip_implemented_method;
mod t04_override_for_trait;
mod t05_override_for_trait_impl;
mod t06_override_for_trait_where;
mod t07_use_mod_path_for_trait_conflict;
mod t08_use_mod_alias_for_trait_conflict;
mod t09_trait_method_conflict;
mod t10_async_method;

#[test]
fn compile_error_check() {
    let t = trybuild::TestCases::new();
    t.compile_fail("src/compile_errors/*.rs");
}
