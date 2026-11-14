//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_manager::{
    ENTERPRISE_FILE, KPIO_FILE, OPAL_1_FILE, OPAL_2_FILE, OPALITE_FILE, PYRITE_1_FILE, PYRITE_2_FILE, RUBY_FILE,
};
use sed_manager_gui_slint as ui;
use sed_spec::ssc::Ssc;
use slint::platform::PointerEventButton::Left;

use crate::driver::Driver;

pub const TEST_PASSWORD: &str = "test-password-1234";
pub const USER1_PASSWORD: &str = "user1-password-1234";

pub async fn scan_devices(ui: &Driver<ui::MainWindow>) {
    ui.single_click("scan-button", Left).await.unwrap();
    ui.check_condition("scan-button", |handle| handle.accessible_enabled().unwrap_or(true))
        .await
        .unwrap();
}

pub async fn select_virtual_device(ui: &Driver<ui::MainWindow>, ssc: Ssc) {
    let file_name = match ssc {
        Ssc::Enterprise => ENTERPRISE_FILE,
        Ssc::OpalV1 => OPAL_1_FILE,
        Ssc::OpalV2 => OPAL_2_FILE,
        Ssc::Opalite => OPALITE_FILE,
        Ssc::PyriteV1 => PYRITE_1_FILE,
        Ssc::PyriteV2 => PYRITE_2_FILE,
        Ssc::Ruby => RUBY_FILE,
        Ssc::KeyPerIO => KPIO_FILE,
    };
    let accessible_id = format!("device-side-bar-/dev/virtual_device/{file_name}");
    ui.check_appearance(&accessible_id).await.unwrap();
    // The devices in the side bar get reordered as their identities get loaded
    // in the background after the scan. An asynchronous mouse click could land
    // on another device that moved under the cursor mid-click, so the device is
    // selected synchronously, by its element rather than by its position.
    ui.invoke_default_action(&accessible_id).unwrap();
    ui.check_appearance("activity-take-ownership").await.unwrap();
}

pub async fn take_ownership(ui: &Driver<ui::MainWindow>) {
    // Switch to the "Take ownership" activity and fill in the password form.
    ui.single_click("activity-take-ownership", Left).await.unwrap();
    ui.enter_text("new-password", TEST_PASSWORD).await.unwrap();
    ui.enter_text("repeat-password", TEST_PASSWORD).await.unwrap();

    // Submit and wait for the success toast. The submit buttons are disabled while the TPer
    // or the session is busy, so they are invoked synchronously right after they're found
    // enabled, as opposed to an asynchronous mouse click, during which they may get disabled.
    ui.check_enabled("take-ownership-submit").await.unwrap();
    ui.invoke_default_action("take-ownership-submit").unwrap();
    ui.check_appearance_toast("Taken ownership").await.unwrap();

    // A successful action triggers a background re-discovery, which navigates the sidebar
    // back to the "Discovery" activity once it completes. Wait for that before switching
    // activities again, or the switch may be undone by the pending re-discovery.
    ui.check_disappearance("new-password").await.unwrap();
}

pub async fn activate_locking(ui: &Driver<ui::MainWindow>) {
    // Switch to the "Activate locking" activity and submit the owner password.
    ui.single_click("activity-activate-locking", Left).await.unwrap();
    ui.enter_text("activate-locking-password", TEST_PASSWORD).await.unwrap();
    ui.check_enabled("activate-locking-submit").await.unwrap();
    ui.invoke_default_action("activate-locking-submit").unwrap();
    ui.check_appearance_toast("Locking activated").await.unwrap();
    ui.check_disappearance("activate-locking-password").await.unwrap();
}

pub async fn revert_admin_sp(ui: Driver<ui::MainWindow>) {
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
}

pub async fn revert_locking_sp(ui: &Driver<ui::MainWindow>) {
    // Switch to the "Revert device" activity and revert just the locking SP.
    ui.single_click("activity-revert-device", Left).await.unwrap();
    ui.enter_text("revert-password", TEST_PASSWORD).await.unwrap();
    ui.check_enabled("revert-submit").await.unwrap();
    ui.invoke_default_action("revert-submit").unwrap();
    ui.check_appearance_toast("Reverted device successfully").await.unwrap();
    ui.check_disappearance("revert-password").await.unwrap();
    // Reverting the locking SP disables locking again, so "Activate locking" reappears.
    ui.check_appearance("activity-activate-locking").await.unwrap();
}
