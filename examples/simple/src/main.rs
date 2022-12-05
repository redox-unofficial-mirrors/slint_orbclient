#[allow(clippy::all)]
mod generated_code {
    slint::include_modules!();
}

pub use generated_code::*;

pub fn main() {
    slint_orbclient::init_config(
        slint_orbclient::Config::default()
            .width(600)
            .height(400)
            .resizable(true)
            .title("Simple"),
    );

    let app = App::new();

    app.run();
}
