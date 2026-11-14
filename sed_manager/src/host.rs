//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::path::{Path, PathBuf};
use std::sync::Arc;

use sed_async::PolyRuntime;
use sed_device::{list_storage_devices, open_storage_device};
use tracing::instrument;

use crate::{Device, Error};

pub const VIRTUAL_DEVICE_DIR: &str = "/dev/virtual_device";

pub const ENTERPRISE_FILE: &str = "enterprise";
pub const OPAL_1_FILE: &str = "opal_1";
pub const OPAL_2_FILE: &str = "opal_2";
pub const OPALITE_FILE: &str = "opalite";
pub const PYRITE_1_FILE: &str = "pyrite_1";
pub const PYRITE_2_FILE: &str = "pyrite_2";
pub const RUBY_FILE: &str = "ruby";
pub const KPIO_FILE: &str = "kpio";

/// The entry point to access the storage devices of the host computer system.
///
/// You might wonder why we need a `struct` for this. To separate the UI and the
/// backend into separate processes for security, this would be turned into an
/// RPC service, requiring a `struct`. This is just prep work here.
#[derive(Debug)]
pub struct Host {
    runtime: Arc<PolyRuntime>,
}

impl Host {
    /// Open the host computer for managing SEDs.
    ///
    /// Further services created by the host will use the provided async runtime.
    pub fn new(runtime: Arc<PolyRuntime>) -> Self {
        Self { runtime }
    }

    /// List the physical drives present in the system.
    /// See [`list_storage_devices`] to learn more.
    #[instrument(level = "info", skip(self), ret, err)]
    pub async fn list_devices(&self) -> Result<Vec<PathBuf>, Error> {
        #[allow(unused_mut)]
        let mut devices = list_storage_devices().await?;
        #[cfg(feature = "virtual_device")]
        {
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{ENTERPRISE_FILE}").into());
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{OPAL_1_FILE}").into());
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{OPAL_2_FILE}").into());
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{OPALITE_FILE}").into());
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{PYRITE_1_FILE}").into());
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{PYRITE_2_FILE}").into());
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{RUBY_FILE}").into());
            devices.push(format!("{VIRTUAL_DEVICE_DIR}/{KPIO_FILE}").into());
        };
        Ok(devices)
    }

    /// Opens a drive by the path identifying it.
    /// See [`open_storage_device`] to learn more.
    #[instrument(level = "info", skip(self, path), err)]
    pub async fn open_device(&self, path: impl AsRef<Path>) -> Result<Device, Error> {
        #[cfg(feature = "virtual_device")]
        if path.as_ref().parent().is_some_and(|dir| dir == VIRTUAL_DEVICE_DIR) {
            use sed_spec::ssc::Ssc;
            use sed_virtual_device::VirtualDeviceBuilder;

            let ssc_name = path.as_ref().file_name().and_then(|name| name.to_str());
            let ssc = ssc_name.and_then(|ssc_name| match ssc_name {
                ENTERPRISE_FILE => Some(Ssc::Enterprise),
                OPAL_1_FILE => Some(Ssc::OpalV1),
                OPAL_2_FILE => Some(Ssc::OpalV2),
                OPALITE_FILE => Some(Ssc::Opalite),
                PYRITE_1_FILE => Some(Ssc::PyriteV1),
                PYRITE_2_FILE => Some(Ssc::PyriteV2),
                RUBY_FILE => Some(Ssc::Ruby),
                KPIO_FILE => Some(Ssc::KeyPerIO),
                _ => None,
            });
            if let Some(ssc) = ssc {
                let device = VirtualDeviceBuilder::new().with_path(path.as_ref().into()).with_ssc(ssc).build();
                return Ok(Device::new(Box::new(device), self.runtime.clone()).await);
            }
        }
        let device = open_storage_device(path).await?;
        Ok(Device::new(device, self.runtime.clone()).await)
    }
}
