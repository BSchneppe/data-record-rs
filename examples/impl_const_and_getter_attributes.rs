use data_record::DataRecord;

#[derive(DataRecord)]
#[datarecord(name = "Attributes", impl_getter = true, impl_const = true)]
#[datarecord_getter_attr(cfg(not(target_os = "tvos")))]
#[datarecord_getter_impl_attr(cfg(not(target_os = "tvos")))]
#[datarecord_const_attr(cfg(not(target_os = "tvos")))]
#[datarecord_const_impl_attr(cfg(not(target_os = "tvos")))]
#[datarecord_const_impl_method_attr(cfg(not(target_os = "tvos")))]
struct Example3 {
    a: u32,
    b: u32,
}

fn main() {
    let example = Example3::new(1, 2);
    assert_eq!(example.a(), 1);
    assert_eq!(example.b(), 2);
}
