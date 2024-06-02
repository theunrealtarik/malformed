#[cfg(windows)]
fn build() {
    use winres::*;

    WindowsResource::new()
        .set_icon("./assets/icon/ico.ico")
        .compile()
        .unwrap();
}

fn main() {
    build();
}
