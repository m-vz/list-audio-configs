use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    cpal::default_host()
        .default_input_device()
        .expect("No default input device")
        .supported_input_configs()
        .expect("No supported input configs")
        .for_each(|config| println!("{config:?}"));
}
