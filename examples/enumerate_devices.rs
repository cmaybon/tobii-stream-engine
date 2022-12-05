extern crate tobii_stream_engine;
use tobii_stream_engine::api::Api;

fn main() {
    println!("{}", Api::get_api_version_string());
    let mut api = Api::new();
    api.print_devices_to_stdout();
}
