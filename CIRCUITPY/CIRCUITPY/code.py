import digitalio
import board
import audiomp3
import audiopwmio
import protocol

CLEAR_PIN = board.GP13
# (led, switch)
GREEN_TEAM_PINS = [(board.GP28, board.GP27), (board.GP26, board.GP22), (board.GP17, board.GP16), (board.GP19, board.GP18), (board.GP21, board.GP20)] # todo what is pin for last red team?
RED_TEAM_PINS = [(board.GP1, board.GP0), (board.GP2, board.GP3), (board.GP5, board.GP4), (board.GP7, board.GP8), (board.GP11, board.GP10)]

clear = digitalio.DigitalInOut(CLEAR_PIN)
clear.switch_to_input(pull=digitalio.Pull.DOWN)

red_led = digitalio.DigitalInOut(board.GP6)
red_led.switch_to_output(value=False)

green_led = digitalio.DigitalInOut(board.GP12)
green_led.switch_to_output(value=False)

# led, switch, buzz_protocol, lockout_protocol
red_pins = []
green_pins = []
for i in range(len(RED_TEAM_PINS)):
    red_pins.append((
        digitalio.DigitalInOut(RED_TEAM_PINS[i][0]),
        digitalio.DigitalInOut(RED_TEAM_PINS[i][1]),
        protocol.RED_TEAM_BUZZES[i],
        protocol.RED_TEAM_LOCKOUTS[i]
    ))
for i in range(len(GREEN_TEAM_PINS)):
    green_pins.append((
        digitalio.DigitalInOut(GREEN_TEAM_PINS[i][0]),
        digitalio.DigitalInOut(GREEN_TEAM_PINS[i][1]),
        protocol.GREEN_TEAM_BUZZES[i],
        protocol.GREEN_TEAM_LOCKOUTS[i]
    ))
# print(red_pins)

for led, switch, _, _ in red_pins + green_pins:
    led.switch_to_output(value=False)
    led.direction = digitalio.Direction.OUTPUT
    switch.switch_to_input(pull=digitalio.Pull.DOWN)
    led.value = True

import time
red_led.value = True
green_led.value = True
audio = audiopwmio.PWMAudioOut(board.GP14)

def play_startup():
    b = audiomp3.MP3Decoder(open("./startup.mp3", "rb"))
    audio.play(b)
play_startup()
while audio.playing:
    pass

for led, _, _, _ in red_pins + green_pins:
    led.value = False
red_led.value = False
green_led.value = False


# https://learn.adafruit.com/mp3-playback-rp2040/pico-mp3
import supervisor
import sys
def check_for_serial_message():
    if supervisor.runtime.serial_bytes_available:
        return sys.stdin.read(1)
    return ''

def buzzer_loop():
    while True:
        audio.stop()
        led_pin, lockout_exclude = await_buzz(red_pins, green_pins)    
        led_pin.value = True
        t = time.localtime()
        # print(f"buzz! at {t.tm_hour}:{t.tm_min}:{t.tm_sec}")
        play_buzz_tone() # todo: different tones for different teams
        time.sleep(1.5) # delay - not sure why, but sometimes a clear happens right after a buzz
        await_clear(lockout_exclude)
        print(protocol.CLEAR)
        led_pin.value = False
        red_led.value = False
        green_led.value = False

def await_buzz(red, green):
    while True:
        for led, switch, buzz_protocol, lockout_protocol in red + green:
            #print(switch.value)
            if switch.value == True:
                time.sleep(0.01)
                if switch.value == False:
                    continue
                print(buzz_protocol)
                if buzz_protocol >= "F":
                    red_led.value = True
                else:
                    green_led.value = True
                
                return led, lockout_protocol
def await_clear(last_buzz = ''):
    while audio.playing:
        pass
    # duplicated
    locked_out_already = [last_buzz]
    clear.switch_to_output()
    clear.value = False
    clear.switch_to_input(pull=digitalio.Pull.DOWN)
    while clear.value == False:
        m = check_for_serial_message()
        if m == 'x':
            # Forced clear
            return do_clear()
        if m == '2':
            play_two_bits()
        if not audio.playing:
            audio.stop()
        for led, switch, _, lockout_protocol in red_pins + green_pins:
            if switch.value == True and lockout_protocol not in locked_out_already:
                time.sleep(0.1)
                switch.switch_to_output(value=False)
                switch.switch_to_input(pull=digitalio.Pull.DOWN)
                if switch.value != True:
                    continue
                print(lockout_protocol)
                locked_out_already.append(lockout_protocol)
    return do_clear()

def do_clear():
    for led, switch, _, _ in red_pins + green_pins:
        led.value = False
        led.switch_to_output(value=False)
        switch.switch_to_output(value=False)
        switch.value = False
        switch.switch_to_input(pull=digitalio.Pull.DOWN)

    clear.switch_to_output(value=False)
    clear.value = False
    clear.switch_to_input(pull=digitalio.Pull.DOWN)
    return True

def play_buzz_tone():
    buzz_tone = audiomp3.MP3Decoder(open("./buzz.mp3", "rb"))
    audio.play(buzz_tone)

def play_two_bits():
    b = audiomp3.MP3Decoder(open("./two-bits.mp3", "rb"))
    audio.play(b)


print("go")
buzzer_loop()
