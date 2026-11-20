//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_manager_gui_slint as ui;
use sed_spec::ssc::Ssc;

use crate::ui_conv::{FromBackend, IntoUi};

impl FromBackend<Ssc> for ui::Ssc {
    fn from_backend(value: Ssc) -> Self {
        match value {
            Ssc::Enterprise => ui::Ssc::Enterprise,
            Ssc::KeyPerIO => ui::Ssc::KeyPerIo,
            Ssc::OpalV1 => ui::Ssc::Opal1,
            Ssc::OpalV2 => ui::Ssc::Opal2,
            Ssc::Opalite => ui::Ssc::Opalite,
            Ssc::PyriteV1 => ui::Ssc::Pyrite1,
            Ssc::PyriteV2 => ui::Ssc::Pyrite2,
            Ssc::Ruby => ui::Ssc::Ruby,
        }
    }
}

impl FromBackend<Option<Ssc>> for ui::Ssc {
    fn from_backend(value: Option<Ssc>) -> Self {
        match value {
            Some(ssc) => ssc.into_ui(),
            None => ui::Ssc::None,
        }
    }
}
