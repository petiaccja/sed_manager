# Testing plan for loading the ACEs

This plan covers integration tests for the `load` functions in
`sed_manager/src/locking_config/access_control/aces.rs`. The tests go into
`sed_manager/tests/integration/locking_config/access_control.rs` and run against
the virtual device. The pure `*_source` functions are already covered by unit
tests in `aces.rs`.

## Prerequisites

1. Implement `LockingConfigSession::get_access_control` as
   `Aces::load(&self.session, Ssc::from(&self.spec.ssc_desc))`, keeping the
   Enterprise guard. The integration tests can't call `Aces::load` directly:
   `Ssc` is private, and the `Session` inside a `LockingConfigSession` is not
   accessible. Going through `get_access_control` tests the public API.
2. Export `AceExpr` from `access_control/mod.rs`, so the tests can name its
   variants.
3. Move the shared helpers (`setup`, `login_admin1`, `NEW_SID_PASSWORD`,
   `range_uid`, `user_uid`) from `locking_config_session.rs` to
   `locking_config/mod.rs` or a `common` module.
4. Add helpers:
    - `login_as(tper, authority, password)`, and `login_anybody(tper)` (`ANYBODY`
      without a password).
    - `setup_user1(admin1_session)`: enable User1 via `set_authorities` and set
      its password via `change_password`.
    - `set_device_ace(device, ace, expr)`: overwrite an ACE's `BooleanExpr`
      through `device.modify`.
    - `fixed(expr)`, `loaded(expr)`, `assumed(expr)` shorthands.
    - A builder for the expected Opal 2 preconfiguration, parametrized by how
      editable slots are wrapped (`loaded` or `assumed`).

## Virtual device behavior

| Behavior                                                                                 | Consequence for the tests                                                                                                         |
| ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------- |
| Opal 2 only: 8 ranges + GlobalRange, 4 admins, 8 users                                   | No Enterprise, Pyrite, Opal 1, Opalite, or Ruby paths.                                                                            |
| `Get` never returns `NotAuthorized`, it filters out unpermitted columns                  | The `FieldNotReturned` → `Assumed` path is exercised. The `NotAuthorized` arms are unreachable.                                   |
| Only a `K_AES_256` table; no AccessControl `Get` row for the keys; `Mode` is `None`      | The key table probe takes `FieldNotReturned` → present for 256-bit, and `InvalidParameter` → absent for 128-bit. `Ok` is never hit. |
| The MBR byte table has no permission check                                               | The MBR probe always succeeds unless the MBR is emptied.                                                                          |
| Session start doesn't check `Enabled`                                                    | Don't rely on it: enable User1 properly, so the tests stay valid if the virtual device starts enforcing it.                       |
| `Next` on the Authority table also returns `Anybody`, `Admins`, `Users`                   | The `authorities` map has 15 entries, three of which are all `Fixed(empty)`. Assert this explicitly.                              |

## Virtual device improvements

These would make the tests more faithful to real devices, and are worth adding
before writing the tests:

- **`K_AES` mode reading**: `Get` rows using `ACE_K_AES_Mode` on the key
  objects, and actual `Mode` values. This matches the specification and
  exercises the probe's `Ok` path.
- **Optional `K_AES_128` table**: allows testing a device with both key tables,
  and with only the `K_AES_128` table.
- **Optional MBR**: make `Locking::mbr` an `Option` to model a device without
  shadow MBR, instead of emptying the vector.
- **Optional `K_AES_256` table**: same as the MBR, to model a device without
  media encryption.
- **`NotAuthorized` for `Get`**: a test-only switch to return `NotAuthorized`
  instead of filtering columns, to exercise those arms in `AceSource::load` and
  the probes.
- **Enforcing `Enabled` at session start**: disabled authorities should not be
  able to authenticate.
- **Other SSCs**: preconfigurations for Pyrite, Opal 1, Opalite, Ruby, and
  Enterprise, to test SSC-specific loading.

## Test cases

Each test starts with a fresh device via `setup()`.

### A. `load_as_admin_preconfig`

Log in as Admin1 and compare the whole `Aces` against the expected
preconfiguration with a single `eq`:

- **Global**: `read_ace_table`, `read_authority_table`, `read_data_store`,
  `write_data_store` are `Loaded(Admins)`; `set_ace_expressions` is
  `Fixed(Admins)`.
- **MBR** (`Some`): `set_enable_to_dor` and `upload` are `Fixed(Admins)`;
  `set_done` is `Loaded(Admins)`.
- **Locking ranges**: the keys are exactly GlobalRange and Range1 to Range8.
  Each has `read_start_to_active_key`, `set_read_locked`, `set_write_locked` as
  `Loaded(Admins)`; `set_start_to_lor` as `Fixed(Admins)`; `erase_128: None`;
  `erase_256: Some(Loaded(Admins))`.
- **Authorities** (15 entries):
    - Admin1: `set_enabled` is `Fixed(empty)`, `set_common_name` and `set_pin`
      are `Fixed(Admins)`.
    - Admin2 to Admin4: `set_enabled` is `Fixed(Admins)`.
    - User{n}: `set_enabled` is `Fixed(Admins)`, `set_common_name` is
      `Loaded(Admins)`, `set_pin` is `Loaded(Admins OR User{n})`.
    - `Anybody`, `Admins`, `Users`: all `Fixed(empty)`.

### B. `load_as_user_preconfig`

Admin1 enables User1 and sets its password, then log in as User1.

- The same structure as A, but every editable slot is `Assumed(default)`,
  because `ACE_ACE_Get_All` is Admins-only.
- `Fixed` slots are unchanged, `mbr` is still `Some`, and `erase_256` is still
  `Some`. This shows the probes don't depend on the authority.

### C. `load_as_anybody_preconfig`

Same expectation as B, but as `Anybody`. This shows both probes work without
authentication. A, B, and C can share one `#[rstest]` over the login function
and the expected wrapper.

### D. `load_as_user_with_ace_read_access`

Widen `ACE_ACE_Get_All` to `Admins OR Users` via `set_device_ace`, then log in
as User1. All editable slots are now `Loaded`, and `read_ace_table` is
`Loaded(Admins OR Users)`. This verifies that loading depends on what the device
returns, not on who is logged in.

### E. `load_maps_every_editable_ace_to_its_slot`

The end-to-end equivalent of the unit tests where each ACE grants a different
user. Give every editable ACE on the device a unique expression (e.g.
`USER.get(k)` with a distinct `k` per ACE; the range has 2048 UIDs and the
virtual device doesn't validate operands). Log in as Admin1, load, and compare
against the expected `Aces` built from the same mapping. This catches wrong
index arithmetic (e.g. `Range{n}` vs. `Range{n}_GenKey`), swapped fields, and
mixups between the global range and the other ranges.

### F. `load_ignores_device_values_of_fixed_aces`

Change `ACE_Locking_Admins_RangeStartToLOR`, `ACE_ACE_Set_BooleanExpression`,
and `ACE_MBRControl_Admins_Set` on the device, then load as Admin1. The result
is still `Fixed(Admins)`, documenting that fixed slots are never read.

### G. `load_without_mbr`

Remove the MBR from the device (or empty it until the MBR is optional in the
virtual device). `aces.mbr` is `None`.

### H. `load_without_key_tables`

Remove the `K_AES_256` table (or replace it with an empty table). `erase_128`
and `erase_256` are `None` for every range. Optionally, check that
`Permissions::effective_of(admin1, &aces)` gives `erase == None`, the
Pyrite-like path end to end.

### I. `load_fails_on_missing_editable_ace`

Remove an editable ACE object (e.g. `ACE_Locking_Range8_Set_RdLocked`) from the
device's ACE table. `get_access_control` returns an error. This pins down the
chosen strict behavior: a missing ACE is a device deviation and is reported.
Revisit if loading is made lenient.

### J. `permissions_from_loaded_aces` (optional)

A short end-to-end check of `Permissions::effective_of` on the results of A and
B:

- As Admin1: everything is granted, except Admin1's `set_enabled`.
- As User1 (preconfiguration): own `set_pin` is granted; User2's `set_pin`,
  `set_read_locked` of every range, and `erase` (`Some(false)`) are denied.

### Cases enabled by the virtual device improvements

- **Both key tables**: `erase_128` and `erase_256` are both `Some`, and erase
  permission is the AND of the two (e.g. change only one of them on the
  device).
- **Only `K_AES_128`**: `erase_128` is `Some`, `erase_256` is `None`.
- **Probe `Ok` path**: with `Mode` readable, the key table probe succeeds
  without `FieldNotReturned`.
- **`NotAuthorized`**: editable slots become `Assumed`, and the probes report
  absent tables.

## Not coverable without other SSCs in the virtual device

Enterprise, Pyrite, Opal 1, Opalite, and Ruby are only covered by the
`*_source` unit tests until the virtual device supports their preconfiguration.

## Order

Implement A, B, and E first. A and B lock in the two main loading modes, and E
is the test most likely to catch a real wiring bug.
