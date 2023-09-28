// inspired by: https://stackoverflow.com/a/56389650

macro_rules! generate_struct {
    ($name:ident {$($field_name:ident : $field_type:ty),+}) => {
        struct $name { $($field_name: $field_type),+ }
        impl $name {
            fn show_vars(&self) {
            $(
            let field_name = stringify!($field_name);
            let field_type = stringify!($field_type);
               println!("{:?} : {:?}", field_name, field_type);
            )*
            }
        }
    };
}

generate_struct! { Foo { bar: i32, baz: String } }

fn main() {
    let foo = Foo{bar: 123, baz: "abc".to_string()};
    foo.show_vars();
}
