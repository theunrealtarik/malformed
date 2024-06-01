extern crate winres;

#[cfg(windows)]
fn build() {
    winres::WindowsResource::new()
        .set_icon("./assets/icon/ico.ico")
        .compile()
        .unwrap();
}

fn main() {
    build();
}
