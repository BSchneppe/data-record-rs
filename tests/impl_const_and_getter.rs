use data_record::DataRecord;
extern crate data_record;

#[derive(DataRecord)]
#[datarecord(
    name = "MyCustomTrait",
    constructor_name = "build",
    impl_getter,
    impl_const
)]
struct Example {
    a: u32,
    b: u32,
}

#[test]
fn should_create_struct_with_impls() {
    let example = Example::build(1, 2);
    assert_eq!(example.a(), 1);
    assert_eq!(example.b(), 2);
}
