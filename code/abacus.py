# check pins
from machine import Pin
import time
pin = Pin(15, Pin.IN)
led = Pin(28, Pin.OUT)
def await_buzz(pins):
    [reset(pin) for pin in pins]
    led.off()
    while True:
        for pin in pins:
            if pin.value() == 1: # Buzz!
                led.toggle() # todo reduce the current here
                reset(pin)
                return pin
            
def reset(pin):
    pin.init(mode=pin.OUT)
    pin.value(0)
    pin.init(mode=pin.IN)

import time
    
def buzzer_loop():
    input_pins = [Pin(16, Pin.IN)]
    led_pins = [Pin(17, Pin.OUT)]
    for pin in input_pins:
        reset(pin)
    for pin in led_pins:
        pin.off()
    while True:
        # TODO: turn off LED
        buzzed_pin = await_buzz(input_pins)
        buzz_time = time.time_ns()
        led = led_pins[input_pins.index(buzzed_pin)]
        led.on()
        year, month, mday, hour, minute, second, weekday, yearday = time.localtime(buzz_time // 10**9)
        ms = (buzz_time % 10**9)//10**6
        print(f"buzz at {hour}:{minute}:{second} + {ms} millisec")
        buzz_chime(60000)

        reset(buzzed_pin)
        time.sleep(1)

# See https://www.instructables.com/Respberry-Pi-Pico-W-Generating-Tones-With-Programm
# And also just https://datasheets.raspberrypi.com/pico/raspberry-pi-pico-python-sdk.pdf pg. 19-20
# takes 10 cycles

@rp2.asm_pio(set_init=rp2.PIO.OUT_LOW)
def blink():
    wrap_target()
    set(pins, 1) [31]
    nop() [31]
    nop() [31]
    nop() [31]
    nop() [31]
    set(pins, 0) [31]
    nop() [31]
    nop() [31]
    nop() [31]
    nop() [31]
    wrap()

import machine
# Instantiate a state machine with the blink program, at 2000Hz, with set bound to Pin(25) (LED
#sm = rp2.StateMachine(0, blink, freq=500, set_base=Pin(17))
# Run the state machine for 3 seconds. The LED should blink.


def spkr(f=1000, t= 0.2):
    sm = rp2.StateMachine(0, blink, freq=f, set_base=Pin(15))
    sm.active(1)
    time.sleep(t)
    sm.active(0)
    Pin(15, Pin.OUT).low()
def buzz_chime(base):
    spkr(base, t=0.12)
    spkr(base+10000, t=0.12)
    spkr(base+20000, t=0.12)
    spkr(base+30000, t=0.12)
    spkr(base+40000, t=0.4)
