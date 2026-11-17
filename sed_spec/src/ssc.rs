//L-----------------------------------------------------------------------------
//L Copyright (C) Péter Kardos
//L Please refer to the full license distributed with this software.
//L-----------------------------------------------------------------------------

use std::fmt;

use sed_packet::discovery::SscFeatureDescriptor;

use crate::{
    methods::Properties,
    objects::SecurityProviderRef,
    uid_registry::{enterprise, kpio, opal_2},
};

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

impl Ssc {
    /// Returns the secondary SP of the SSC. For KPIO, this is the KPIO SP,
    /// for Enterprise and the Opal family, this is the Locking SP with the
    /// appropriate UID.
    pub fn secondary_sp(self) -> SecurityProviderRef {
        use Ssc::*;
        match self {
            Enterprise => enterprise::admin::sp::LOCKING,
            OpalV1 | OpalV2 | Opalite | PyriteV1 | PyriteV2 | Ruby => opal_2::admin::sp::LOCKING,
            KeyPerIO => kpio::admin::sp::KEY_PER_IO,
        }
    }

    /// The properties assumed at the beginning of token streaming, before a
    /// `Properties` exchange.
    pub fn initial_properties(self) -> Properties {
        use Ssc::*;
        match self {
            Enterprise => Properties::INITIAL_ENTERPRISE,
            OpalV1 | OpalV2 | Opalite | PyriteV1 | PyriteV2 | Ruby => Properties::INITIAL_OPAL_FAMILY,
            KeyPerIO => Properties::INITIAL_KPIO,
        }
    }

    /// Returns if the PSID feature set is mandatory for the SSC. The device may
    /// still support the PSID feature set, so it must be checked at runtime.
    pub fn psid_mandatory(self) -> bool {
        use Ssc::*;
        match self {
            Enterprise | OpalV1 | PyriteV1 => false,
            OpalV2 | Opalite | PyriteV2 | Ruby | KeyPerIO => true,
        }
    }

    /// Returns whether the SSC makes use of the `Activate` method to enable the
    /// secondary SP.
    pub fn activate_supported(self) -> bool {
        use Ssc::*;
        match self {
            Enterprise => false,
            OpalV1 | OpalV2 | Opalite | PyriteV1 | PyriteV2 | Ruby | KeyPerIO => true,
        }
    }

    /// Returns if the SSC supports the `RevertSP` method. Not to be confused
    /// with the `Revert` method.
    pub fn revert_sp_supported(self) -> bool {
        use Ssc::*;
        match self {
            Enterprise | KeyPerIO => false,
            OpalV1 | OpalV2 | Opalite | PyriteV1 | PyriteV2 | Ruby => true,
        }
    }

    /// Returns if the SSC supports the `Revert` method invoked by the `SID`
    /// authority. Devices that do not support this may still support the
    /// `Revert` method invoked by the `PSID` if they implement the PSID feature
    /// set.
    pub fn revert_by_sid_supported(self) -> bool {
        use Ssc::*;
        match self {
            Enterprise => false,
            OpalV1 | OpalV2 | Opalite | PyriteV1 | PyriteV2 | Ruby | KeyPerIO => true,
        }
    }

    /// Returns what method the SSC uses for erasing locking ranges. For devices
    /// that do not have locking ranges or the ranges cannot be explicitly erased,
    /// [`EraseMethod::None`] is returned.
    pub fn erase_method(self) -> EraseMethod {
        use Ssc::*;
        match self {
            Enterprise => EraseMethod::Erase,
            OpalV1 | OpalV2 | Opalite | Ruby => EraseMethod::GenKey,
            PyriteV1 | PyriteV2 | KeyPerIO => EraseMethod::None,
        }
    }
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

/// The method the device uses the erase locking ranges.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EraseMethod {
    /// Enterprise uses the `Erase` method on the locking range's UID.
    Erase,
    /// The Opal family uses the `GenKey` method on the `K_AES_*` table entries,
    /// provided that the device has media encryption.
    GenKey,
    /// KPIO does not have locking ranges, and Pyrite devices don't support media
    /// encryption.
    None,
}
