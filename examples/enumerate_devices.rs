extern crate tobii_stream_engine;
use tobii_stream_engine::api::Api;

fn main() {
    let mut api = Api::new();
    println!("{}", Api::get_api_version_string());
    println!("{}", api.system_clock());
    api.print_devices_to_stdout();
}
