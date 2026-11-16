//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_manager_gui_slint as ui;
use sed_spec::ssc::Ssc;

use crate::ui_conv::FromBackend;

impl FromBackend<Option<Ssc>> for ui::Ssc {
    fn from_backend(value: Option<Ssc>) -> Self {
        match value {
            Some(Ssc::Enterprise) => ui::Ssc::Enterprise,
            Some(Ssc::KeyPerIO) => ui::Ssc::KeyPerIo,
            Some(Ssc::OpalV1) => ui::Ssc::Opal1,
            Some(Ssc::OpalV2) => ui::Ssc::Opal2,
            Some(Ssc::Opalite) => ui::Ssc::Opalite,
            Some(Ssc::PyriteV1) => ui::Ssc::Pyrite1,
            Some(Ssc::PyriteV2) => ui::Ssc::Pyrite2,
            Some(Ssc::Ruby) => ui::Ssc::Ruby,
            None => ui::Ssc::None,
        }
    }
}
