// Main Code for The BSOD
// src/main.rs

// Rewriting Author: Mizumo-prjkt
// Special Credits: https://github.com/peewpw/Invoke-BSOD

// TBH this is a pain to develop on a non-VS Code environment.
// I am using kate and LSP is giving me a fucking aneurisym.
// Linter is also very delayed and had to frequently clean diagnostic results
// to get it to display errors properly everytime.

use std::process;

use clap::Parser;

mod argument;
mod panic;
mod behavior;

use argument::ArgParser;

use crate::behavior::traitbehavior;


unsafe extern "system" {
    // Detatch from Terminal
    fn FreeConsole() -> i32 ;
}

fn main() {
    // Parse args right away!
    let _args = ArgParser::parse();

    if _args.quiet {
        unsafe {
            FreeConsole();
        }
        println!("If you see this, lol. To be honest, this software is supposed to be for educational use only, and did not mean harm. But if you did not intend to have this software, then that means someone probably is trying to compromise your hardware. Also hello. uhhhhh sir. If you see this via some form of Hex editor or String checks from Linux or idk what util from windows, then i will say this, this software was not meant for harm. But if this software was seen being disrupting the wide web, then i apologize. I did not mean harm. But ill say this, please lock the NT Kernel to only microsoft to have access to it at this point.");
    }
    if _args.possible_codes {
        println!("
        BSOD Codes

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

        Note that default is 00 (STATUS_ACCESS_DENIED)
             The middle table is what you really need to use
             -l 00 (or any code)
             or
             --bsod-code a6 (or any code)
        Visit:
        https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/596a1078-e883-4972-9bbc-49e60bebca55
        for more info.

        ");
        process::exit(0);
    }


    traitbehavior(&_args);

}

