pub mod name {
    use derive_mode::builder;

    #[derive(builder)]
    pub struct MyStruct {
        pub myattr1: String,
        pub myattr2: String,
    }

    #[derive(builder)]
    pub struct YourStruct {
        pub yourattr1: String,
        pub yourattr2: String,
        pub yourattr3: String,
    }

}

fn main() {
    let ms = name::MyStructBuilder::new()
        .with_myattr1("aaa".to_string())
        .with_myattr2("bbb".to_string())
        .build();

    assert_eq!(ms.myattr1, "aaa".to_string());
    assert_eq!(ms.myattr2, "bbb".to_string());

    let ys = name::YourStructBuilder::new()
        .with_yourattr1("aaa".to_string())
        .with_yourattr2("bbb".to_string())
        .with_yourattr3("ccc".to_string())
        .build();

    assert_eq!(ys.yourattr1, "aaa".to_string());
    assert_eq!(ys.yourattr2, "bbb".to_string());
    assert_eq!(ys.yourattr3, "ccc".to_string());
}
