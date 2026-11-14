//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::fmt;

use sed_packet::discovery::SscFeatureDescriptor;

/// One of the security subsystem classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Ssc {
    Enterprise,
    OpalV1,
    OpalV2,
    Opalite,
    PyriteV1,
    PyriteV2,
    Ruby,
    KeyPerIO,
}

impl fmt::Display for Ssc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Ssc::Enterprise => "Enterprise",
            Ssc::OpalV1 => "OpalV1",
            Ssc::OpalV2 => "OpalV2",
            Ssc::Opalite => "Opalite",
            Ssc::PyriteV1 => "PyriteV1",
            Ssc::PyriteV2 => "PyriteV2",
            Ssc::Ruby => "Ruby",
            Ssc::KeyPerIO => "KeyPerIO",
        };
        f.write_str(s)
    }
}

impl From<SscFeatureDescriptor> for Ssc {
    fn from(value: SscFeatureDescriptor) -> Self {
        Self::from(&value)
    }
}

impl From<&SscFeatureDescriptor> for Ssc {
    fn from(value: &SscFeatureDescriptor) -> Self {
        match value {
            SscFeatureDescriptor::Enterprise(_) => Self::Enterprise,
            SscFeatureDescriptor::OpalV1(_) => Self::OpalV1,
            SscFeatureDescriptor::OpalV2(_) => Self::OpalV2,
            SscFeatureDescriptor::Opalite(_) => Self::Opalite,
            SscFeatureDescriptor::PyriteV1(_) => Self::PyriteV1,
            SscFeatureDescriptor::PyriteV2(_) => Self::PyriteV2,
            SscFeatureDescriptor::Ruby(_) => Self::Ruby,
            SscFeatureDescriptor::KeyPerIO(_) => Self::KeyPerIO,
        }
    }
}
