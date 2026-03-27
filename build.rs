use std::env;

use winresource::WindowsResource;

fn main() {
    slint_build::compile("ui/app-window.slint").expect("Slint build failed");

    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
            .set_icon("res/CTR_logo.ico")
            .compile()
            .expect("Windows build failed");
    }
}
