pub struct Foo {
    a_value: u8,
}

pub struct Bar {
    foo: Foo,
}

impl Bar {
    pub fn use_my_foo(&self) -> u8 {
        self.foo.a_value()
    }
}
