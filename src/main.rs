use env_logger::{Builder, Target};

mod ll;
mod device;

fn main() {
    Builder::from_default_env().target(Target::Stdout).init();

    ll::getATPR().expect("ATPR");
}
