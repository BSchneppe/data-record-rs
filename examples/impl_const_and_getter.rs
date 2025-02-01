use data_record::DataRecord;

#[derive(DataRecord)]
#[datarecord(name = "AnotherOne", impl_getter = true, impl_const = true)]
struct Example2 {
    a: u32,
    b: u32,
}

fn main() {
    let example = Example2::new(1, 2);
    assert_eq!(example.a(), 1);
    assert_eq!(example.b(), 2);
}
