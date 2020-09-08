# NUCLEO-F303K8 Sample Application
Drone-OS firmware example for STM32 NUCLEO-F303K8 board.

## Difficulty level
Basic 'blinky' type application in combination with an emulated userbutton and a 
dynamic clock tree configuration that changes when PB5 is connected shortly to 3V3.

## Summary
- Configure the clock tree to run the mcu at 64, 32 and 8 MHz dynamically
  selectable at run-time.
- Configure 1 GPIO output pin to drive the on-board green user led.
  (only possible after desoldering SB15 and connecting the resistor to PB4 instead.
- Write log message to SWO output (only possible after desoldering SB15).
- Configure the EXTI interrupt for the gpio that is assigned to the button.
- Listen to the systick and to the button click event stream simultaneously.

This firmware is written with the 'official' Drone-OS crates. No additional
crates were used other than those normally used by Drone-OS.

## Toolchain
The project is currently dependent on nightly-2020-04-30. It will be upgraded
to the latest nightly as soon as the corresponding Drone-OS crates are released.

## Hardware modifications needed
Unfortunately, the Nucleo STM32F303K8 has connected the TRACESWO SB3 pin to the
onboard LED. If you want to use the SWO logging feature, you need to cut that
connection by removing the SB15 zero-ohm resistor.

## Debug probe.
The Nucleo STM32F303K8 board has an ST-Link v2.1 integrated on the board. 
It works with openocd for flashing and debugging with gdb.
Unfortunately, the board is missing a connection from the F303 mcu to the ST-Link mcu.
With some soldering skils and a patch wire, that connection can be added and
the logging will be forwarded by the ST-Link. There is a simplier solution for
those who are not eager to solder wires to the mcu pins:
The SWO output from pin PB3 can be sent to the PC via any UART/USB adapter. 
The SB3 pin must be wired to the RX pin of the adapter. As SB3 was originally 
connected to the SB15 solder bridge, the wire can be soldered to the board
without touching the mcu.

In Drone.toml, the endpoint must be defined matching the virtual COM-port for the adapter.
Example:
serial-endpoint = "/dev/ttyUSB0"
Finally, you will get the log output by executing 'just log' command.

## Troubleshooting
Sometimes, the openocd/USB/embedded ST-LINK/SWD debug connection only starts up correctly after pressing RESET button on the target (and keep it pressed while you execute 'just flash'). Than release the button and try again.

## License
Licensed under either of

Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)
MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
