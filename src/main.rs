#![windows_subsystem = "windows"]

const VERSION: &str = env!("CARGO_PKG_VERSION");
use slint::{ComponentHandle, Timer, TimerMode};
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let ui = Window::new()?;
    let cps_timer = Timer::default();
    let autoclickers_timer = Timer::default();

    {
        let ui_handle = ui.clone_strong().as_weak();
        let ui = ui_handle.unwrap();
        ui.global::<AboutMetadata>().set_version(VERSION.into());
    }

    {
        let ui_handle = ui.clone_strong().as_weak();
        cps_timer.start(TimerMode::Repeated, std::time::Duration::from_secs(5), move || {
            let ui = ui_handle.unwrap();
            let calculated_cps = ui.get_cps_counter() / 5;
            ui.set_current_cps(calculated_cps);
            ui.set_cps_counter(0);
        });
    }

    {
        let ui_handle = ui.clone_strong().as_weak();
        autoclickers_timer.start(TimerMode::Repeated, std::time::Duration::from_secs(1), move || {
            let ui = ui_handle.unwrap();
            ui.set_clicks(ui.get_clicks() + ui.get_upgradecps());
        })
    }

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_clicks(ui.get_clicks() + 1);
            ui.set_cps_counter(ui.get_cps_counter() + 1);
        }
    });

    ui.global::<UpgradesLogic>().on_request_autoclicker_purchase({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            if ui.get_clicks() >= ui.global::<UpgradesLogic>().get_autoclicker_price() {
                ui.set_clicks(ui.get_clicks() - ui.global::<UpgradesLogic>().get_autoclicker_price());
                ui.global::<UpgradesLogic>().set_autoclickers(ui.global::<UpgradesLogic>().get_autoclickers() + 1);
                ui.set_upgradecps(ui.get_upgradecps() + 1);
            } else {
                return;
            }
        }
    });

    ui.global::<UpgradesLogic>().on_request_grandma_purchase({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            if ui.get_clicks() >= ui.global::<UpgradesLogic>().get_grandma_price() {
                ui.set_clicks(ui.get_clicks() - ui.global::<UpgradesLogic>().get_grandma_price());
                ui.global::<UpgradesLogic>().set_grandmas(ui.global::<UpgradesLogic>().get_grandmas() + 1);
                ui.set_upgradecps(ui.get_upgradecps() + 2);
            } else {
                return;
            }
        }
    });

    ui.global::<UpgradesLogic>().on_request_robot_purchase({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            if ui.get_clicks() >= ui.global::<UpgradesLogic>().get_robot_price() {
                ui.set_clicks(ui.get_clicks() - ui.global::<UpgradesLogic>().get_robot_price());
                ui.global::<UpgradesLogic>().set_robots(ui.global::<UpgradesLogic>().get_robots() + 1);
                ui.set_upgradecps(ui.get_upgradecps() + 4);
            } else {
                return;
            }
        }
    });

    ui.run()
}
