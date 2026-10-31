# Access control configuration and gating locking configuration UI

## Relevant ACEs for the Opal-family

Meaning of the columns:

- **Editable**: the ACE has an AccessControl `Set` row, gated by `ACE_ACE_Set_BooleanExpression`, so an Admin can rewrite its `BooleanExpr`. The `Columns` of every ACE are fixed.
- **Assigning permissions**: a permission that the end-user can assign to an authority through the UI.
- **Gating UI commands**: the object for which the ACE determines if the currently authenticated authority can read or write it
- **SSCs**: O1 = Opal 1.00, O2 = Opal 2.x, Ol = Opalite, R = Ruby, P = Pyrite 2.

### Locking ranges

| ACE                                                            | Default | Columns                           | Editable | Used in ACL of                                | Assigning permissions  | Gating UI commands         | SSCs                                                               |
| -------------------------------------------------------------- | ------- | --------------------------------- | -------- | --------------------------------------------- | ---------------------- | -------------------------- | ------------------------------------------------------------------ |
| `ACE_Locking_{GlobalRange,Range{n}}_Get_RangeStartToActiveKey` | Admins  | RangeStart…LockOnReset, ActiveKey | Y        | `Locking_*.Get`                               | **Read range fields**  | Display locking range info | all (Range{n}: O1, O2, R-ALR)                                      |
| `ACE_Locking_{GlobalRange,Range{n}}_Set_RdLocked`              | Admins  | ReadLocked                        | Y        | `Locking_*.Set`                               | **Read-unlock** range  | Read lock/unlock toggle    | all (Range{n}: O1, O2, R-ALR)                                      |
| `ACE_Locking_{GlobalRange,Range{n}}_Set_WrLocked`              | Admins  | WriteLocked                       | Y        | `Locking_*.Set`                               | **Write-unlock** range | Write lock/unlock toggle   | all (Range{n}: O1, O2, R-ALR)                                      |
| `ACE_K_AES_{128,256}_{GlobalRange,Range{n}}_GenKey`            | Admins  | All                               | Y        | `K_AES_*_Key.GenKey`                          | **Erase** range        | Erase range button         | O1, O2, Ol, R. Not P.                                              |
| `ACE_Locking_GlblRng_Admins_Set`                               | Admins  | ReadLockEnabled…LockOnReset       | N        | `Locking_GlobalRange.Set`                     | –                      | Modify global range        | all                                                                |
| `ACE_Locking_Admins_RangeStartToLOR`                           | Admins  | RangeStart…LockOnReset            | N        | `Locking_Range{n}.Set` (shared by all ranges) | –                      | Modify non-global range    | O2, R-ALR. O1 calls it `…RangeStartToLocked`, without LockOnReset. |

### Shadow MBR

| ACE                            | Default | Columns                   | Editable | Used in ACL of                     | Assigning permissions | Gating UI commands                              | SSCs                                                        |
| ------------------------------ | ------- | ------------------------- | -------- | ---------------------------------- | --------------------- | ----------------------------------------------- | ----------------------------------------------------------- |
| `ACE_MBRControl_Set_DoneToDOR` | Admins  | Done, DoneOnReset         | Y        | `MBRControl.Set`                   | **MBR Done** cell     | Global: MBR Done toggle                         | all. O1 calls it `ACE_MBRControl_Set_Done`, with Done only. |
| `ACE_MBRControl_Admins_Set`    | Admins  | Enable, Done, DoneOnReset | N        | `MBRControl.Set`                   | –                     | Global: MBR enable toggle (and Done for Admins) | all                                                         |
| `ACE_Admin`                    | Admins  | All                       | N        | `MBR.Set` (also `ThisSP.RevertSP`) | –                     | Global: MBR upload button; revert-SP action     | all                                                         |

### Authorities and passwords

| ACE                              | Default               | Columns                                    | Editable | Used in ACL of                     | Assigning permissions             | Gating UI commands                                                                                 | SSCs                      |
| -------------------------------- | --------------------- | ------------------------------------------ | -------- | ---------------------------------- | --------------------------------- | -------------------------------------------------------------------------------------------------- | ------------------------- |
| `ACE_Authority_Get_All`          | Admins                | All                                        | Y        | `Authority_*.Get`                  | Global: "read authorities"        | Display authority tab                                                                              | all                       |
| `ACE_User{n}_Set_CommonName`     | Admins                | CommonName                                 | Y        | `User{n}.Set`                      | Per-user "rename" cell            | Rename user                                                                                        | O2, R                     |
| `ACE_C_PIN_User{n}_Set_PIN`      | **Admins OR User{n}** | PIN                                        | Y        | `C_PIN_User{n}.Set`                | Per-user "change password" button | Change user password. This is the only default that grants a non-Admin anything.                   | all (Ol, P: User1–2 only) |
| `ACE_Authority_Set_Enabled`      | Admins                | Enabled                                    | N        | `Admin{2..}.Set`, `User{n}.Set`    | –                                 | Authority enable toggle. **Admin1 is not in any `Set_Enabled` ACL**, so nobody can disable Admin1. | all                       |
| `ACE_Admins_Set_CommonName`      | Admins                | CommonName                                 | N        | `Admin*.Set`, `Locking_*.Set`      | –                                 | Rename admin / range                                                                               | O2, R                     |
| `ACE_Anybody_Get_CommonName`     | Anybody               | UID, CommonName                            | N        | `Authority_*.Get`, `Locking_*.Get` | –                                 | Lets every authority list names                                                                    | O2, R                     |
| `ACE_C_PIN_Admins_Get_All_NOPIN` | Admins                | UID, CharSet, TryLimit, Tries, Persistence | N        | `C_PIN_*.Get`                      | –                                 | Try counters / limits                                                                              | all                       |
| `ACE_C_PIN_Admins_Set_PIN`       | Admins                | PIN                                        | N        | `C_PIN_Admin*.Set`                 | –                                 | Change admin password                                                                              | all                       |

### ACE table, DataStore and generic

| ACE                             | Default | Columns     | Editable | Used in ACL of                                                              | Assigning permissions         | Gating UI commands                                                                     | SSCs |
| ------------------------------- | ------- | ----------- | -------- | --------------------------------------------------------------------------- | ----------------------------- | -------------------------------------------------------------------------------------- | ---- |
| `ACE_ACE_Get_All`               | Admins  | All         | Y        | `ACE_*.Get`                                                                 | Global: "read access control" | Whether the current assignments and gating for the current authority can be determined | all  |
| `ACE_DataStore_Get_All`         | Admins  | All         | Y        | `DataStore.Get`                                                             | Global: "read DataStore"      | DataStore read (if exposed)                                                            | all  |
| `ACE_DataStore_Set_All`         | Admins  | All         | Y        | `DataStore.Set`                                                             | Global: "write DataStore"     | DataStore write (if exposed)                                                           | all  |
| `ACE_ACE_Set_BooleanExpression` | Admins  | BooleanExpr | N        | `<editable ACE>.Set`                                                        | –                             | Display access control tab                                                             | all  |
| `ACE_Anybody`                   | Anybody | All         | N        | `*.Next`, `Authenticate`, `MBRControl.Get`, `MBR.Get`, `LockingInfo.Get`, … | –                             | – (always granted)                                                                     | all  |

## Assigning permissions

The permissions that are important to make configurable are divided into global,
per-range, and per-authority groups. The global and per-range ACE exprs can be set
from the GUI, the per-authority ones are only read (see below).

### Global permissions

| Permission               | ACE                            | Default | Importance                                                                 |
| ------------------------ | ------------------------------ | ------- | -------------------------------------------------------------------------- |
| Read ACE table           | `ACE_ACE_Get_All`              | Admins  | Enables accurate filtering of locking config UI tabs; filtering in the PBA |
| Read authority table     | `ACE_Authority_Get_All`        | Admins  | Let users display the authority tab and change their password through it   |
| Set MBR to done          | `ACE_MBRControl_Set_DoneToDOR` | Admins  | Let users unshadow the MBR after boot                                      |
| Read the DataStore table | `ACE_DataStore_Get_All`        | Admins  | Let users read chain-unlock configuration from the DataStore table         |

Notes:

- Writing to the `DataStore` table is not something that has to be configurable,
  because it's enough if admins can write it. If chain-unlocking is implemented,
  we want to know `ACE_DataStore_Set_All` for UI gating.

### Per-range permissions

| Permission         | ACE                                                                                                    | Default | Importance                                                      |
| ------------------ | ------------------------------------------------------------------------------------------------------ | ------- | --------------------------------------------------------------- |
| Read locking range | `ACE_Locking_GlobalRange_Get_RangeStartToActiveKey` / `ACE_Locking_Range{n}_Get_RangeStartToActiveKey` | Admins  | Lets users display locking range information beyond common name |
| Set read locked    | `ACE_Locking_GlobalRange_Set_RdLocked` / `ACE_Locking_Range{n}_Set_RdLocked`                           | Admins  | Lets users unlock a range for reading, core functionality       |
| Set write locked   | `ACE_Locking_GlobalRange_Set_WrLocked` / `ACE_Locking_Range{n}_Set_WrLocked`                           | Admins  | Lets users unlock a range for writing, core functionality       |
| Erase range        | `ACE_K_AES_{128,256}_GlobalRange_GenKey` / `ACE_K_AES_{128,256}_Range{n}_GenKey`                       | Admins  | Lets users erase a range                                        |

Notes:

- When granting an authority `Set_RdLocked` or `Set_WrLocked` for a range, the
  editor should implicitly grant `Get_RangeStartToActiveKey` for the same range
  as well. Without it, the authority can't read `ReadLocked` / `WriteLocked`, so
  the checkbox shows "not available" (and the compare-and-swap would fail), even
  though the authority is permitted to set the value.
- The editor should guard against removing `Admins` from the key ACEs that have
  no fixed Admin-only fallback, otherwise admins lock themselves out without a
  way to see the problem:
    - `ACE_ACE_Get_All`: admins can't read the policy anymore, and can only fix it
      blind via `ACE_ACE_Set_BooleanExpression`.
    - `ACE_Authority_Get_All`: admins can't read the authorities' columns.
    - `Get_RangeStartToActiveKey`: admins can't read (and therefore can't modify)
      the locking range.
    - `GenKey`: admins can't erase the range.
  `Set_RdLocked`, `Set_WrLocked`, and `Set_DoneToDOR` don't need a guard, because
  admins still have access through the fixed `*_Admins_*` ACEs.

### Per-authority permissions

These permissions cannot be assigned from the GUI. It could be implemented, but
it's not important enough to worth the effort. They still need to be read in case
they were modified through another application.

| Permission           | ACE                          | Default           | Importance                                          |
| -------------------- | ---------------------------- | ----------------- | --------------------------------------------------- |
| Set user common name | `ACE_User{n}_Set_CommonName` | Admins            | Let users set their own or each other's common name |
| Set user PIN         | `ACE_C_PIN_User{n}_Set_PIN`  | Admins OR User{n} | Let users set their own or each other's password    |

## Gating UI commands

The ACEs listed below control how data is displayed on the locking config view.
The effect of the change on the displayed data is described in the tables, the
default behavior can be assumed from the effect.

The source column describes how the gating information is obtained, which
determines where it lives in the Rust data structures. Each piece should be loaded
from the TPer at most once:

- **Editable**: editable on the TPer (see "Assigning permissions"). Loaded from
  the TPer once, and shared between assigning permissions and gating, even for the
  per-authority ACEs that are only read, not assigned, in the GUI.
- **Fixed**: hardcoded from the SSC, not loaded from the TPer at all.
- **Gating only**: loaded from the TPer, but used only for gating, not for editing.
- **Trial read**: not evaluated from the ACE, but derived from which columns the
  TPer returned for the object.

### Global commands

| Command                   | ACEs                                                          | Default | Effect                                                                               | Source                              |
| ------------------------- | ------------------------------------------------------------- | ------- | ------------------------------------------------------------------------------------ | ----------------------------------- |
| Read ACE table            | `ACE_ACE_Get_All`                                             | Admins  | When false: hide access control tab, assume all editable ACE exprs are their default | Trial read of the ACE objects       |
| Read authority table      | `ACE_Authority_Get_All`                                       | Admins  | When false: show placeholder for most columns of the authority                       | Trial read of the authority objects |
| Set MBR to enabled        | `ACE_MBRControl_Admins_Set`                                   | Admins  | When true: enable MBR enabled switch                                                 | Fixed                               |
| Set MBR to done           | `ACE_MBRControl_Admins_Set` OR `ACE_MBRControl_Set_DoneToDOR` | Admins  | When true: enable MBR done switch                                                    | Fixed OR Editable                   |
| Upload MBR & `RevertSP`   | `ACE_Admin`                                                   | Admins  | When true: enable MBR upload button (& possible future `RevertSP` button)            | Fixed                               |
| Write the DataStore table | `ACE_DataStore_Set_All`                                       | Admins  | FUTURE: When true: enable control to configure chain-unlocking                       | Gating only                         |

Note:

- Assuming the default values when `ACE_ACE_Get_All` is false will likely be mostly
  right, and it will block users from everything except for changing their
  own password.

### Per-range commands

| Command            | ACEs                                                                                                   | Default | Effect                                                     | Source                                  |
| ------------------ | ------------------------------------------------------------------------------------------------------ | ------- | ---------------------------------------------------------- | --------------------------------------- |
| Read locking range | `ACE_Locking_GlobalRange_Get_RangeStartToActiveKey` / `ACE_Locking_Range{n}_Get_RangeStartToActiveKey` | Admins  | When false: show placeholder for most columns of the range | Trial read of the locking range objects |
| Set read locked    | `ACE_Locking_GlobalRange_Set_RdLocked` / `ACE_Locking_Range{n}_Set_RdLocked` OR _Modify_               | Admins  | When true: enable set read-locked checkbox                 | Editable OR Fixed                       |
| Set write locked   | `ACE_Locking_GlobalRange_Set_WrLocked` / `ACE_Locking_Range{n}_Set_WrLocked` OR _Modify_               | Admins  | When true: enable set write-locked checkbox                | Editable OR Fixed                       |
| Erase range        | `ACE_K_AES_{128,256}_GlobalRange_GenKey` / `ACE_K_AES_{128,256}_Range{n}_GenKey`                       | Admins  | When true: enable erase button                             | Editable                                |
| Modify range       | _Modify_ `ACE_Locking_GlblRng_Admins_Set` / `ACE_Locking_Admins_RangeStartToLOR`                       | Admins  | When true: enable all locking range UI setter elements     | Fixed                                   |

### Per-authority commands

| Command         | ACEs                                                       | Default                                     | Effect                                                        | Source                                                  |
| --------------- | ---------------------------------------------------------- | ------------------------------------------- | ------------------------------------------------------------- | ------------------------------------------------------- |
| Set enabled     | `ACE_Authority_Set_Enabled`                                | Admins                                      | When true: enable the authority enable checkbox               | Fixed (ACL membership queried per authority, see below) |
| Set common name | `ACE_Admins_Set_CommonName` / `ACE_User{n}_Set_CommonName` | Admins (O2, R), nobody (O1, Ol, P) / Admins | When true: enable the common name text edit for the authority | Fixed by SSC / Editable                                 |
| Set PIN         | `ACE_C_PIN_Admins_Set_PIN` / `ACE_C_PIN_User{n}_Set_PIN`   | Admins / Admins OR User{n}                  | When true: enable the set password button for the authority   | Fixed / Editable                                        |

Notes:

- `Set enabled`: the ACE itself is fixed, but whether it applies depends on the
  authority's `Set` ACL (Admin1's ACL doesn't contain it, so nobody can disable
  Admin1). The ACL is queried per authority, but the ACE expr is never loaded.
