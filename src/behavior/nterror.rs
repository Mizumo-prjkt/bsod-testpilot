// src/behavior/nterror.rs

/*
 * NT Codes
 *
 * Note: Default should be 00
 *
 * | Error Code   | BSOD.exe trigger | Description                         |
 * |--------------|------------------|-------------------------------------|
 * | 0xc000000D   | 3a               | STATUS_INVALID_PARAMETER            |
 * | 0xc0000022   | 00               | STATUS_ACCESS_DENIED                |
 * | 0xc0000018   | 7a               | STATUS_CONFLICTING_ADDRESS          |
 * | 0xc0000010   | bf               | STATUS_INVALID_DEVICE_REQUEST       |
 * | 0xc000000B   | c1               | STATUS_INVALID_CID                  |
 * | 0xc0000004   | a2               | STATUS_INFO_LENGTH_MISMATCH         |
 * | 0xc000002D   | 3b               | STATUS_NOT_COMMITED                 |
 * | 0xc0000046   | b2               | STATUS_MUTANT_NOT_OWNED             |
 * | 0xc000009A   | 6e               | STATUS_INSUFFICIENT_RESOURCES       |
 * | 0xc0000244   | c4               | STATUS_AUDIT_FAILED                 |
 * | 0xc014000F   | a3               | STATUS_ACPI_INVALID_DATA            |
 * | 0xc0140010   | a4               | STATUS_ACPI_INVALID_REGION          |
 * | 0xc0140019   | a5               | STATUS_ACPI_INVALID_TABLE           |
 * | 0xc0140021   | a6               | STATUS_ACPI_POWER_REQUEST_FAILED    |
 *
 * any of error codes that are not added here, will default to 00
 *
 * NOTE to dev:
 * _EXEFLAG should be &str, since this is only for the executable. Do not convert to u8.
 * Thank you.
 * Maybe in the future update???
 *
 * BSOD CODE REFERENCE:
 * https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/596a1078-e883-4972-9bbc-49e60bebca55
 */

// Default
pub const DEFAULT_BSOD:                             u32  = 0xc0000022;
// May say its not mentioned, but maybe in future?
pub const DEFAULT_BSOD_EXEFLAG:                     &str = "00";

// Other BSOD Flags and related bsod.exe code
pub const STATUS_INVALID_PARAMETER:                 u32  = 0xc000000d;
pub const STATUS_INVALID_PARAMETER_EXEFLAG:         &str = "3a";
pub const STATUS_ACCESS_DENIED:                     u32  = 0xc0000022;
pub const STATUS_ACCESS_DENIED_EXEFLAG:             &str = "00";
pub const STATUS_CONFLICTING_ADDRESS:               u32  = 0xc0000018;
pub const STATUS_CONFLICTING_ADDRESS_EXEFLAG:       &str = "7a";
pub const STATUS_INVALID_DEVICE_REQUEST:            u32  = 0xc0000010;
pub const STATUS_INVALID_DEVICE_REQUEST_EXEFLAG:    &str = "bf";
pub const STATUS_INVALID_CID:                       u32  = 0xc000000b;
pub const STATUS_INVALID_CID_EXEFLAG:               &str = "c1";
pub const STATUS_INFO_LENGTH_MISMATCH:              u32  = 0xc0000004;
pub const STATUS_INFO_LENGTH_MISMATCH_EXEFLAG:      &str = "a2";
pub const STATUS_NOT_COMMITED:                      u32  = 0xc000002d;
pub const STATUS_NOT_COMMITED_EXEFLAG:              &str = "3b";
pub const STATUS_MUTANT_NOT_OWNED:                  u32  = 0xc0000046;
pub const STATUS_MUTANT_NOT_OWNED_EXEFLAG:          &str = "b2";
pub const STATUS_INSUFFICIENT_RESOURCES:            u32  = 0xc000009a;
pub const STATUS_INSUFFICIENT_RESOURCES_EXEFLAG:    &str = "6e";
pub const STATUS_AUDIT_FAILED:                      u32  = 0xc0000244;
pub const STATUS_AUDIT_FAILED_EXEFLAG:              &str = "c4";
pub const STATUS_ACPI_INVALID_DATA:                 u32  = 0xc014000f;
pub const STATUS_ACPI_INVALID_DATA_EXEFLAG:         &str = "a3";
pub const STATUS_ACPI_INVALID_REGION:               u32  = 0xc0140010;
pub const STATUS_ACPI_INVALID_REGION_EXEFLAG:       &str = "a4";
pub const STATUS_ACPI_INVALID_TABLE:                u32  = 0xc0140019;
pub const STATUS_ACPI_INVALID_TABLE_EXEFLAG:        &str = "a5";
pub const STATUS_ACPI_POWER_REQUEST_FAILED:         u32  = 0xc0140021;
pub const STATUS_ACPI_POWER_REQUEST_FAILED_EXEFLAG: &str = "a6";

//
