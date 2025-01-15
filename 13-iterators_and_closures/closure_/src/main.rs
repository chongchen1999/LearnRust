fn main() {
    closure_::simple_demo::run();
    closure_::capture_demo::immut_ref();
    closure_::capture_demo::mut_ref();
    closure_::move_ownership::move_ownership_demo();
    closure_::closure_traits::closure_fnonce();
    closure_::closure_traits::closure_fnmut();
    closure_::closure_traits::closure_fn();
}
