use data_record::DataRecord;

#[derive(DataRecord)]
#[datarecord(name = "MyCustomTrait", constructor_name = "build")]
struct Example {
    a: u32,
    b: u32,
}

impl MyCustomTraitGetter for Example {
    fn a(&self) -> u32 {
        self.a
    }

    fn b(&self) -> u32 {
        self.b
    }
}
impl MyCustomTraitConstructor for Example {
    fn build(a: u32, b: u32) -> Self {
        Self { a, b }
    }
}

fn main() {
    let example = Example::build(1, 2);
    assert_eq!(example.a(), 1);
    assert_eq!(example.b(), 2);
}
