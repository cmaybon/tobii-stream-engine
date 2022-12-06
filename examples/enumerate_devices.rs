extern crate tobii_stream_engine;
use tobii_stream_engine::api::{Api, Device, FieldOfUse};

fn main() {
    let mut api = Api::new();
    println!("{}", Api::get_api_version_string());
    println!("{}", api.system_clock());
    let urls = api.device_urls();
    println!("{:?}", urls);
    let mut device = Device::new(&api, urls[0].clone(), FieldOfUse::Interactive).unwrap();
    device.get_info();
    println!("{:?}", device);
}
