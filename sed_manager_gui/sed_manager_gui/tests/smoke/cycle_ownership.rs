//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::sync::Arc;
use std::time::Duration;

use sed_async::{PolyRuntime, SlintRuntime};
use sed_manager::Host;
use sed_manager_gui::app::App;
use sed_manager_gui::toast::ToastQueue;
use sed_manager_gui_slint as ui;
use sed_spec::ssc::Ssc;
use slint::ComponentHandle as _;
use slint::platform::PointerEventButton::Left;

use crate::driver::Driver;
use crate::shared_steps::{activate_locking, scan_devices, select_virtual_device, take_ownership};
use crate::test_utils::run_test;

const TEST_PASSWORD: &str = "test-password-1234";

/// This test cycles the ownership of the device, starting from factory ->
/// owned -> activated -> owned -> factory.
///
/// This test exercises the following functionalities:
/// - Take ownership
/// - Activate locking
/// - Revert (locking & everything)
#[test]
fn cycle_ownership() {
    run_test(|| async {
        let runtime = Arc::new(PolyRuntime::Slint(SlintRuntime));
        let ui = ui::MainWindow::new().unwrap();
        let notification_queue = ToastQueue::new(ui.clone_strong());
        let host = Arc::new(Host::new(runtime.clone()));
        let _app = App::new(ui.clone_strong(), notification_queue, host, runtime.clone());

        let ui = Driver::new(ui, Duration::from_secs(5));

        scan_devices(&ui).await;
        select_virtual_device(&ui, Ssc::OpalV2).await;
        take_ownership(&ui).await;
        activate_locking(&ui).await;

        // Switch to the "Revert device" activity and revert just the locking SP.
        ui.single_click("activity-revert-device", Left).await.unwrap();
        ui.enter_text("revert-password", TEST_PASSWORD).await.unwrap();
        ui.check_enabled("revert-submit").await.unwrap();
        ui.invoke_default_action("revert-submit").unwrap();
        ui.check_appearance_toast("Reverted device successfully").await.unwrap();
        ui.check_disappearance("revert-password").await.unwrap();
        // Reverting the locking SP disables locking again, so "Activate locking" reappears.
        ui.check_appearance("activity-activate-locking").await.unwrap();

        // Switch to the "Revert device" activity again, select "Everything" as the scope, and
        // revert the whole device back to its factory state.
        ui.single_click("activity-revert-device", Left).await.unwrap();
        ui.single_click("revert-scope", Left).await.unwrap();
        ui.select_combo_box_item("revert-scope", "Everything").unwrap();
        ui.enter_text("revert-password", TEST_PASSWORD).await.unwrap();
        ui.check_enabled("revert-submit").await.unwrap();
        ui.invoke_default_action("revert-submit").unwrap();
        ui.check_appearance_toast("Reverted device successfully").await.unwrap();
        // Reverting the whole device restores the factory MSID password, so "Take ownership" reappears.
        ui.check_appearance("activity-take-ownership").await.unwrap();
    });
}
