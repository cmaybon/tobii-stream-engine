# tobii-stream-engine
A rust wrapper around the Tobii `tobii-stream-engine.dll`.

Working for `tobii-stream-engine.dll` API version `4.0.0.16` (major.minor.revision.build).

# NOTICE
When using the `tobii-stream-engine.dll` make sure you are properly following the terms of the license you are using.
For most people it will be under the`TOBII TECH GETTING STARTED SOFTWARE DEVELOPMENT` license which can be found online [here](https://developer.tobii.com/wp-content/uploads/2021/01/Tobii-Tech-Getting-Started-SDLA-29-Sept-2020FINAL.pdf).

And make sure to check the license that is included in the zip when you download the `Tobii XR Native SDK`, as one might be more up-to-date than the other.

# Install
0. Install the Tobii runtime for your eye tracker. If you are using the Tobii Eye Tracker 5 it will come with the required software installs.
   If you're using another product you will need to contact Tobii and request the PDK for that product.
1. Add `tobii_stream_engine = "0.1.0"` to your `Cargo.toml` dependencies
2. Download the [Tobii XR Native SDK](https://developer.tobii.com/download-packages/tobii-xr-native-sdk/)
3. From the downloaded SDK, add `stream_engine/lib/tobii/tobii_stream_engine.lib` to `third_party/tobii/`

> NOTE: When building and running a binary, you must have the `tobii_stream_engine.dll` in the working directory of the binary

> NOTE: If you are a [Talon Voice](https://talonvoice.com/) user, you likely disabled any Tobii services when you setup your eye tracker with Talon.  
> You must re-enable:  
>   #### Windows:
>   - `Tobii Runtime Service`


# Example
"Safe" API:
```rust
use tobii_stream_engine::{Api, Device, FieldOfUse, Error};

fn main() {
    let mut api = Api::new();
    let urls = api.device_urls();
    let mut device = api.new_device(urls[0].clone(), FieldOfUse::Interactive).unwrap();
    device.get_info();
}
```

Unsafe bindings:
```rust
use tobii_stream_engine::bindings::core::*;
use std::os::raw::{c_longlong};

fn main() {
   unsafe {
      let mut api_ptr: *mut TobiiApi = std::mem::zeroed();
      assert_eq!(tobii_api_create(&mut api_ptr as *mut *mut TobiiApi,
                                  std::ptr::null_mut(),
                                  std::ptr::null_mut()),
                 TOBII_ERROR_NO_ERROR);
      let mut timestamp: i64 = 0;
      let timestamp_ptr = &mut timestamp as *mut c_longlong;
      assert_eq!(tobii_system_clock(api_ptr, timestamp_ptr), TOBII_ERROR_NO_ERROR);
      println!("{}", timestamp);
   }
}
```

Full examples can be found in [the `examples` directory](examples).

# Credits
- Inspired by [tobii-sys](https://github.com/trishume/tobii-sys) by [trishume](https://github.com/trishume)
