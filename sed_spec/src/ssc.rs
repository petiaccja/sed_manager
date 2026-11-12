//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use sed_packet::discovery::SscFeatureDescriptor;

/// One of the security subsystem classes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
