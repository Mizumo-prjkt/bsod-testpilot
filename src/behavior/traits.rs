// src/behavior/traits.rs


// Trait behavior

use std::env::args;
use std::path::Path;
use std::ptr;
use std::ptr::null;
use std::time::Duration;
use std::time::Instant;
use std::thread;
use std::fs;
use std::env;


use rand::RngExt;
use rand::rng;

use crate::argument::ArgParser;
use crate::behavior::nterror::DEFAULT_BSOD;
// NGL i did not know this would happen
// I come from C, this is fucking retarded
use crate::behavior::nterror::STATUS_ACCESS_DENIED;
use crate::behavior::nterror::STATUS_ACCESS_DENIED_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_INVALID_DATA;
use crate::behavior::nterror::STATUS_ACPI_INVALID_DATA_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_INVALID_REGION_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_INVALID_REGION;
use crate::behavior::nterror::STATUS_ACPI_INVALID_TABLE;
use crate::behavior::nterror::STATUS_ACPI_INVALID_TABLE_EXEFLAG;
use crate::behavior::nterror::STATUS_ACPI_POWER_REQUEST_FAILED;
use crate::behavior::nterror::STATUS_ACPI_POWER_REQUEST_FAILED_EXEFLAG;
use crate::behavior::nterror::STATUS_AUDIT_FAILED;
use crate::behavior::nterror::STATUS_AUDIT_FAILED_EXEFLAG;
use crate::behavior::nterror::STATUS_CONFLICTING_ADDRESS;
use crate::behavior::nterror::STATUS_CONFLICTING_ADDRESS_EXEFLAG;
use crate::behavior::nterror::STATUS_INFO_LENGTH_MISMATCH_EXEFLAG;
use crate::behavior::nterror::STATUS_INFO_LENGTH_MISMATCH;
use crate::behavior::nterror::STATUS_INSUFFICIENT_RESOURCES;
use crate::behavior::nterror::STATUS_INSUFFICIENT_RESOURCES_EXEFLAG;
use crate::behavior::nterror::STATUS_INVALID_CID_EXEFLAG;
use crate::behavior::nterror::STATUS_INVALID_CID;
use crate::behavior::nterror::STATUS_INVALID_DEVICE_REQUEST_EXEFLAG;
use crate::behavior::nterror::STATUS_INVALID_DEVICE_REQUEST;
use crate::behavior::nterror::STATUS_INVALID_PARAMETER;
use crate::behavior::nterror::STATUS_INVALID_PARAMETER_EXEFLAG;
use crate::behavior::nterror::STATUS_MUTANT_NOT_OWNED;
use crate::behavior::nterror::STATUS_MUTANT_NOT_OWNED_EXEFLAG;
use crate::behavior::nterror::STATUS_NOT_COMMITED;
use crate::behavior::nterror::STATUS_NOT_COMMITED_EXEFLAG;

use crate::panic::NtRaiseHardError;
use crate::panic::RtlAdjustPrivilege;




pub fn traitbehavior(argmt: &ArgParser) {
    let answr = &argmt;
    struct BSOD_Point {
        t1: i32,
        t2: u32,
    }
    pub fn TriggerBsod(rsdta: &ArgParser) {
        let resdata = &rsdta;
        let mut code: u32;
        // Check if user allow suprises
        if resdata.suprise_me {
            println!("suprise_me set to: {}", resdata.suprise_me);
            let mut rng_roulette = rand::rng();
            let roulette = rng_roulette.random_range(1..14);
            println!("result_random_spr_res: {}", roulette);
            code = match roulette {
                1  => STATUS_ACCESS_DENIED,
                2  => STATUS_ACPI_INVALID_DATA,
                3  => STATUS_ACPI_INVALID_REGION,
                4  => STATUS_ACPI_INVALID_TABLE,
                5  => STATUS_ACPI_POWER_REQUEST_FAILED,
                6  => STATUS_AUDIT_FAILED,
                7  => STATUS_CONFLICTING_ADDRESS,
                8  => STATUS_INFO_LENGTH_MISMATCH,
                9  => STATUS_INSUFFICIENT_RESOURCES,
                10 => STATUS_INVALID_CID,
                11 => STATUS_INVALID_DEVICE_REQUEST,
                12 => STATUS_INVALID_PARAMETER,
                13 => STATUS_MUTANT_NOT_OWNED,
                14 => STATUS_NOT_COMMITED,
                _ => DEFAULT_BSOD
            };
        } else {
            println!("suprise_me set to: {}", resdata.suprise_me);
            code = match resdata.bsod_code.as_str() {
                STATUS_ACCESS_DENIED_EXEFLAG => {
                    STATUS_ACCESS_DENIED
                }
                STATUS_ACPI_INVALID_DATA_EXEFLAG => {
                    STATUS_ACPI_INVALID_DATA
                }
                STATUS_ACPI_INVALID_REGION_EXEFLAG => {
                    STATUS_ACPI_INVALID_REGION
                }
                STATUS_ACPI_INVALID_TABLE_EXEFLAG => {
                    STATUS_ACPI_INVALID_TABLE
                }
                STATUS_ACPI_POWER_REQUEST_FAILED_EXEFLAG => {
                    STATUS_ACPI_POWER_REQUEST_FAILED
                }
                STATUS_AUDIT_FAILED_EXEFLAG => {
                    STATUS_AUDIT_FAILED
                }
                STATUS_CONFLICTING_ADDRESS_EXEFLAG => {
                    STATUS_CONFLICTING_ADDRESS
                }
                STATUS_INFO_LENGTH_MISMATCH_EXEFLAG => {
                    STATUS_INFO_LENGTH_MISMATCH
                }
                STATUS_INVALID_CID_EXEFLAG => {
                    STATUS_INVALID_CID
                }
                STATUS_INVALID_DEVICE_REQUEST_EXEFLAG => {
                    STATUS_INVALID_DEVICE_REQUEST
                }
                STATUS_INVALID_PARAMETER_EXEFLAG => {
                    STATUS_INVALID_PARAMETER
                }
                STATUS_NOT_COMMITED_EXEFLAG => {
                    STATUS_NOT_COMMITED
                }
                STATUS_INSUFFICIENT_RESOURCES_EXEFLAG => {
                    STATUS_INSUFFICIENT_RESOURCES
                }
                STATUS_MUTANT_NOT_OWNED_EXEFLAG => {
                    STATUS_MUTANT_NOT_OWNED
                }
                _ => {
                    DEFAULT_BSOD
                }
            };
        }
        let mut NTADJ = BSOD_Point { t1: 0, t2: 0 };
        println!("{}", code);
        unsafe {RtlAdjustPrivilege(19, 1, 0, &mut NTADJ.t1 as *mut i32 );}
        unsafe {NtRaiseHardError(code, 0, 0, ptr::null_mut() , 6, &mut NTADJ.t2 as *mut u32);}
    };
    pub fn randomizer(argmt_r: &ArgParser) {
        if argmt_r.randomizer_timer.is_some() {
            // let i = Instant::now();
            // Convert seconds to milliseconds
            let seconds = argmt_r.randomizer_timer.unwrap_or(300);
            // let res_ms = seconds*1000; // Will need elsewhere
            for i in (1..=seconds).rev() {
                println!("{}", i); // This should be supressed on --quiet. But it should be useful for debug
                thread::sleep(Duration::from_secs(1));
            }
            println!("rng_freq: {}", argmt_r.randomizer_freq);
            let mut frequency_rng = argmt_r.randomizer_freq;
            if frequency_rng == 0 || frequency_rng > 10  {
                // Fallback to 10
                frequency_rng = 10;
                // return;
            }
            // Init RNG
            let mut local_rng = rand::rng();
            // Roll RNG
            let roll = local_rng.random_range(1..=frequency_rng);
            println!("roll {}", roll);
            if roll == 1 {
                println!("prevroll: {}", roll);
                // We pass result of clap to TriggerBSOD
                let pass = &argmt_r;
                TriggerBsod(pass);
            } else {
                let rebnd = &argmt_r;
                randomizer(rebnd);
            }

        }
    }
    pub fn erase_dump () {
        // Check if there's Memory dump
        if let Ok(expanded) = shellexpand::env("%SystemRoot%\\MEMORY.DMP") {
            let target_hit = Path::new(expanded.as_ref());
            println!("expnd: {}", target_hit.display());
            if fs::exists(target_hit).unwrap_or(false) {
                println!("Found");
                if let Err(err) = fs::remove_file(target_hit) {
                    println!("ERROR DELETING DUMP! {}", err);
                }
            }
        }
    }
    pub fn timer_trig_bsod(data_argvar: &ArgParser) {
        let data_rcv = &data_argvar;
        let seconds = data_rcv.timeout;
        for i in (1..=seconds).rev() {
            // DEJA vu oh
            println!("tmr {}", i);
            thread::sleep(Duration::from_secs(1));
        }
        // No ramdomizer freq, we bite
        TriggerBsod(data_rcv);
    }
    if argmt.randomizer {
        if argmt.erase_dump {
            println!("errdmp");
            erase_dump();
        }
        println!("Trigger randomizer, detected flag to trigger this...");
        randomizer(answr);
    }
    // Checks if randomizer is false, and also timeout is not empty
    if !argmt.randomizer  {
        if argmt.erase_dump {
            println!("errdmp");
            erase_dump();
        }
        // We go :D
        timer_trig_bsod(answr);
    }

}
