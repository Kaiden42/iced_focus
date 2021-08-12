#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-struct.rs");
    t.pass("tests/02-enum-unit.rs");
    t.pass("tests/03-enum-struct.rs");
    t.pass("tests/04-tuple-struct.rs");
    t.pass("tests/05-tuple-enum.rs");
    t.pass("tests/06-enable-with.rs");
    t.pass("tests/07-vector.rs");
    t.pass("tests/08-array.rs");
    t.pass("tests/09-option.rs");
    t.pass("tests/10-complex.rs");
    t.pass("tests/11-generics.rs");
}
