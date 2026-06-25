# BSOD TestPilot

NOTE!

You are viewing the `havoc` branch, which just directly performs bsod without flags.

This is a BSOD TestPilot Program that allows to trigger Blue Screen of Death (BSOD) on Windows operating system.

This can also be used to test if the program that you are also testing alongside with can cope with this scenaro. Usually, there are tools that can do this. But this one can be run as is, and trigger the BSOD in a safe manner without needing any special tools.

For advanced users who also wants to play hot potato, you can also try tweak the BSOD behavior to your hearts content. 


# Disclaimer

This application is intended for educational purposes only. It is a tool strictly for public awareness, not for causing harm. The developer is not responsible for any damages caused by the use of this application, in every way using this software, or its forked derivatives and the source precursor.

This software was used for personal learning of Rust, and pull of a idiotic joke regarding Rust, that rust, can in fact do BSOD if you let it.


## AI Related Disclaimer

This code was made majorily by Human, the AI involvment is just for teaching me how to use rust. No AI autowrite the whole thing. If you know rust, you know how horrid this source code is.

Still, i have a lot to learn.

# Credits

Special credit to Barrett Adams (peewpw) for the exploit.<br>You can check https://github.com/peewpw/Invoke-BSOD for more info.

This developer is 10000000000x better than me.

# Flags
```
Options:
  -q, --quiet
          Allows quiet execution, detatches from terminal, persists running
  -r, --randomizer
          Allows random Trigger run
  -c, --randomizer-timer <RANDOMIZER_TIMER>
          Randomizer Trigger in seconds. For every 300 seconds (default) of random timer interval, BSOD may trigger or not
  -x, --randomizer-freq <RANDOMIZER_FREQ>
          Chances of BSOD in a random event (1 - 100%, 10 - 10%) [default: 10]
  -t, --timeout <TIMEOUT>
          Time in Seconds to BSOD [default: 1200]
  -d, --erase-dump
          Erase Dump memory if found
      --possible-codes
          Show possible error codes
  -l, --bsod-code <BSOD_CODE>
          Set BSOD Error [default: 00]
  -y, --suprise-me
          Suprise me, random BSOD counter
  -h, --help
          Print help
  -V, --version
          Print version
```
# BSOD Codes that are present:


| Error Code   | BSOD.exe trigger | Description                         |
|--------------|------------------|-------------------------------------|
| 0xc000000D   | 3a               | STATUS_INVALID_PARAMETER            |
| 0xc0000022   | 00               | STATUS_ACCESS_DENIED                |
| 0xc0000018   | 7a               | STATUS_CONFLICTING_ADDRESS          |
| 0xc0000010   | bf               | STATUS_INVALID_DEVICE_REQUEST       |
| 0xc000000B   | c1               | STATUS_INVALID_CID                  |
| 0xc0000004   | a2               | STATUS_INFO_LENGTH_MISMATCH         |
| 0xc000002D   | 3b               | STATUS_NOT_COMMITED                 |
| 0xc0000046   | b2               | STATUS_MUTANT_NOT_OWNED             |
| 0xc000009A   | 6e               | STATUS_INSUFFICIENT_RESOURCES       |
| 0xc0000244   | c4               | STATUS_AUDIT_FAILED                 |
| 0xc014000F   | a3               | STATUS_ACPI_INVALID_DATA            |
| 0xc0140010   | a4               | STATUS_ACPI_INVALID_REGION          |
| 0xc0140019   | a5               | STATUS_ACPI_INVALID_TABLE           |
| 0xc0140021   | a6               | STATUS_ACPI_POWER_REQUEST_FAILED    |

Note that default is 00 (STATUS_ACCESS_DENIED)

The middle table is what you really need to use -l 00 (or any code) or --bsod-code a6 (or any code)

Visit: https://learn.microsoft.com/en-us/openspecs/windows_protocols/ms-erref/596a1078-e883-4972-9bbc-49e60bebca55 for more info.

