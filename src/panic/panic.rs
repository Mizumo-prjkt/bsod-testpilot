// src/panic/panic.rs

// Code to trigger the BSOD

// NOTE: UNSAFE CODE


// Alias
type ParmVar = *mut std::ffi::c_void;

#[link(name = "ntdll")]
unsafe extern "C" {
    // We adjust privilege to allow ourselves to have control on the system shutdown
    pub fn RtlAdjustPrivilege(
      Privilege: i32,
      bEnablePrivilege: i32,
      IsThreadPrivilege: i32,
      PreviousValue: *mut i32
    ) -> u32;

    // We send an ominous error to the Kernel to try trigger
    // the BSOD.
    pub fn NtRaiseHardError(
        ErrorStatus: u32,
        NumberOfParameters: u32,
        UnicodeStringParameterMask: u32,
        Parameters: ParmVar,
        ValidResponseOption: u32,
        Response: *mut u32,
    ) -> u32;
}
