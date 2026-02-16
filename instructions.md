# Detailed notes on how I built the second buzzer set for UCI
1. 3D printed two sets of five buzzers [this gcode file](https://github.com/AnnikaCodes/buzzers/blob/main/3Dmodels/five_buzzers.gcode)
2. Put the jacks ([A-2004-3-4-LP-N-R](https://www.digikey.com/en/products/detail/assmann-wsw-components/A-2004-3-4-LP-N-R/2183632) into the PCB
3. Solder male-male header pins into Raspberry Pi Pico 2.
4. Solder jacks into PCB, 56 Ohm resistor into PCB (R1-R9); R10 got a ~66 Ohm resistor because they're out.
5. For Adafruit STEMMA amp, connect VIN -> 3.3V (4th from top/2nd from bottom where rectangle is at the top), GND -> ground (2nd from the top where the top is the rectangular pin on the audio "A1"), signal -> audio pin (which I think is the rectangle at the top??? but I shall check first)
