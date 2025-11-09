# Notes on constructing ABACUS
- Cable scheme: Black - GND (for LED), Red - +3.3V for switch, Green: +2.whateverV for LED, Yellow: switch.
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
- Should probably put resistors before the LEDs, so that voltage across LED is not too high. Want voltage across to be 2.1V at <= 20mA. Say we provide 3.3V initially. Diode drop is ~2.1 V (up to 2.8V), so V drop across resistor is 3.3V - 2.1 V = 1.2V (or at minimum 0.5V). R = V/I = 1.2V/(20 mA) = 60 ohm, so let's try a 60 ohm resistor. 50 ohm would give

Buzzer wired 11/8: Green jack wire -> LED +, Red jack wire -> LED -, black & yellow => switch
