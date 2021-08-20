use env_logger::{Builder, Target};

mod device;

fn main() {
    Builder::from_default_env().target(Target::Stdout).init();

    device::lsusb();
}
