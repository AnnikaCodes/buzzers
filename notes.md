# Notes on constructing ABACUS
## Notes from the Pi Pico manual
- "If the USB port is the only power source, VSYS and VBUS can be safely shorted together to eliminate the Schottky diode
drop (which improves efficiency and reduces ripple on VSYS)."
   - seems like we should power from USB and short these together
   - could also consider adding batteries, but why?
- Maybe we can use interrupts rather than an active loop --- pg. 16 of https://datasheets.raspberrypi.com/pico/raspberry-pi-pico-python-sdk.pdf --- but I worry about speed.
- I don't think that multicore should help for this project?
- It seems like we can't control USB from MicroPython directly, so we might need to use the REPL for computer-control? Or I2C/SPI/etc?
    - maybe i'm wrong: https://kyuubi0323.github.io/posts/MCU-Pico2/, https://stackoverflow.com/questions/74390514/serial-communication-between-raspberry-pi-pico-and-pc, https://learnembeddedsystems.co.uk/pico-usb-serial-code
    - need to read these ^^^`
