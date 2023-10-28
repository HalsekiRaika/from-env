use from_env::FromEnv;

#[derive(FromEnv)]
pub struct PreInitializeProps {
    host: String,
}

fn main() {
    // no-op
}
