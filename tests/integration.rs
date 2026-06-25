use ars::counted_enum;

counted_enum! {
    pub enum Foo{
        One,
        Two {a: u8, b:u8},
        Three(u8)
    }
}

#[test]
fn counted_enum_works() {
    assert_eq!(Foo::COUNT, 3);

    let _one = Foo::One;
    let _two = Foo::Two { a: 1, b: 2 };
    let _three = Foo::Three(3);
}
