uniffi::setup_scaffolding!("subcrate");

#[derive(uniffi::Record)]
pub struct A {
    pub a: u64,
}
