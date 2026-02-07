# ABACUS: Annika's Buzzers for Ascertaining Competitive Unique Signals
a lower-cost buzzer system for quizbowl competitions

the 3D models were based on https://www.thingiverse.com/thing:12686 and https://www.thingiverse.com/thing:1432052 and https://www.thingiverse.com/thing:1155457

check out [the 'real' website for these!](https://worldbrightening.net/buzzers/)

I don't yet have detailed directions for building it, but you should definitely:
- get a PCB; you can fabricate [these files](https://github.com/AnnikaCodes/buzzers/tree/main/buzzer-pcb), or I'll send you one (almost certainly for cheaper and potentially for free) if you [contact me](mailto:annika@worldbrightening.net)
- buy [stuff from DigiKey](https://www.digikey.com/en/mylists/list/PRWZMR6ARL) (I don't know if this list is complete... you also will definitely need a [speaker](https://www.amazon.com/MakerHawk-Full-Range-Advertising-Connector-Separating/dp/B07GJ4GH67) and some resistors and please, like, [email me about this](mailto:annika@worldbrightening.net) or Discord me or something if you are going to build this! I will help you!)
- print 10 of the [3D models for the buzzers](https://github.com/AnnikaCodes/buzzers/blob/main/3Dmodels/buzzer2.stl) and one of the [3D models for the case](https://github.com/AnnikaCodes/buzzers/blob/main/buzzer-case.stl) (which I still need to move the hole for the cord down to make it actually fit....)
- cut in half your 4p4c cables, push the wire ends through, solder the pushbutton switch to the right wires (this should be RED and GREEN but maybe double check with a multimeter?) and the LED positive lead to black and ground lead to yellow. 
    - be sure to use electrical tape or heatshrink to make it not short itself out!
- glue the buzzers together, and maybe put some superglue around the bottom to stop people from yanking the cord out
- solder all the jacks to the PCB and resistors to limit the current to the LEDs to the board --- whatever you need to reduce 3.3V to a reasonable amount of current given the diode drop of the LED you're using, but I used 50-60 Ohms and mine haven't broken yet.
- solder the Raspberry Pi to the board, maybe with headers if desired
- put red + green LEDs and a switch into the case with wires attached, then attach the wires either directly to the board with solder in the appropriate places, or through headers
- glue the switch into the case, and close the case
- download the software and you're off!