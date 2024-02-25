// https://github.com/slint-ui/slint/issues/1190
#![windows_subsystem = "windows"]

slint::include_modules!();
slint::init_translations!(std::env::current_exe().unwrap().parent().unwrap().join("translations"));


fn main() -> Result<(), slint::PlatformError> {
    let ui = SimpleWindows::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()
}
