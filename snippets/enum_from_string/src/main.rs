pub mod enum_builder {
    use enum_from_string::order;

    #[derive(Debug, order)]
    pub enum Gods {
        Father,
        Mother,
        Maiden,
        Crone,
        Warrior,
        Smith,
        Stranger,
    }
}

fn main() {
    assert_eq!(enum_builder::Gods::Father.order(), 6);
    assert_eq!(enum_builder::Gods::Stranger.order(), 0);
}
