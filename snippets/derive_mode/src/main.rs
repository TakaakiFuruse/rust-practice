pub mod name {
    use derive_mode::builder;

    #[derive(builder)]
    pub struct MyStruct {
        pub myattr1: String,
        pub myattr2: String,
    }
}

fn main() {
    let ms = name::MyStructBuilder::new()
        .with_myattr1("aaa".to_string())
        .with_myattr2("bbb".to_string())
        .build();

    assert_eq!(ms.myattr1, "aaa".to_string());
    assert_eq!(ms.myattr2, "bbb".to_string());
}
