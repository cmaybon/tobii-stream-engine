extern crate tobii_stream_engine;
use tobii_stream_engine::{Api, Device, FieldOfUse};

fn main() {
    let mut api = Api::new();
    let urls = api.device_urls();
    println!("{:?}", urls);
    let mut device = Device::new(&api, &urls[0], FieldOfUse::Interactive).unwrap();
    device.get_info();
    println!("{:?}", device);
}
