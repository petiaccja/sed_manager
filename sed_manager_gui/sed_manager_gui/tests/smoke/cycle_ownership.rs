//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::sync::Arc;
use std::time::Duration;

use rstest::rstest;
use sed_async::{PolyRuntime, SlintRuntime};
use sed_manager::Host;
use sed_manager_gui::app::App;
use sed_manager_gui::toast::ToastQueue;
use sed_manager_gui_slint as ui;
use sed_spec::ssc::Ssc;
use slint::ComponentHandle as _;

use crate::driver::Driver;
use crate::shared_steps::{
    activate_locking, revert_admin_sp, revert_locking_sp, scan_devices, select_virtual_device, take_ownership,
};
use crate::test_utils::run_test;

/// This test cycles the ownership of the device, starting from factory ->
/// owned -> activated -> owned -> factory.
///
/// This test exercises the following functionalities:
/// - Take ownership
/// - Activate locking
/// - Revert (locking & everything)
#[rstest]
#[case::opal_2(Ssc::OpalV2)]
fn cycle_ownership_opal_family(#[case] ssc: Ssc) {
    run_test(move || async move {
        let runtime = Arc::new(PolyRuntime::Slint(SlintRuntime));
        let ui = ui::MainWindow::new().unwrap();
        let notification_queue = ToastQueue::new(ui.clone_strong());
        let host = Arc::new(Host::new(runtime.clone()));
        let _app = App::new(ui.clone_strong(), notification_queue, host, runtime.clone());

        let ui = Driver::new(ui, Duration::from_secs(5));

        scan_devices(&ui).await;
        select_virtual_device(&ui, ssc).await;
        take_ownership(&ui).await;
        activate_locking(&ui).await;
        revert_locking_sp(&ui).await;
        revert_admin_sp(ui).await;
    });
}
