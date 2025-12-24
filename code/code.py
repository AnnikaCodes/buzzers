import digitalio
import board
import audiomp3
import audiopwmio
import protocol

CLEAR_PIN = board.GP9
# (led, switch)
RED_TEAM_PINS = [(board.GP27, board.GP26), (board.GP21, board.GP22), (board.GP16, board.GP17), (board.GP18, board.GP19), (board.GP7, board.GP8)] # todo what is pin for last red team?
GREEN_TEAM_PINS = [(board.GP1, board.GP0),(board.GP2, board.GP3), (board.GP5, board.GP4), (board.GP11, board.GP10), (board.GP13, board.GP12)]

clear = digitalio.DigitalInOut(CLEAR_PIN)
clear.switch_to_input(pull=digitalio.Pull.DOWN)

status_led = digitalio.DigitalInOut(board.GP6)
status_led.switch_to_output(value=False)

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

for led, switch in red_pins + green_pins:
    led.switch_to_output(value=False)
    led.direction = digitalio.Direction.OUTPUT
    switch.switch_to_input(pull=digitalio.Pull.DOWN)

# https://learn.adafruit.com/mp3-playback-rp2040/pico-mp3
audio = audiopwmio.PWMAudioOut(board.GP15)
import time
def buzzer_loop():
    while True:
        audio.stop()
        led_pin = await_buzz(red_pins, green_pins)    
        led_pin.value = True
        t = time.localtime()
        print(f"buzz! at {t.tm_hour}:{t.tm_min}:{t.tm_sec}")
        status_led.value = True
        play_buzz_tone() # todo: different tones for different teams
       
        await_clear()
        led_pin.value = False
        status_led.value = False

def await_buzz(red, green):
    while True:
        for led, switch, buzz_protocol, _ in red + green:
            if switch.value == True:
                time.sleep(0.01)
                #switch.switch_to_input(pull=digitalio.Pull.DOWN)
                if switch.value == False:
                    continue
                print(buzz_protocol)
                return led
def await_clear():
    # duplicated
    locked_out_already = []
    while clear.value == False:
        if not audio.playing:
            audio.stop()
        for led, switch, _, lockout_protocol in red_pins + green_pins:
            if switch.value == True and lockout_protocol not in locked_out_already:
                time.sleep(0.01)
                if switch.value == False:
                    continue
                print(lockout_protocol)
                locked_out_already.append(lockout_protocol)

    for led, switch in red_pins + green_pins:
        led.value = False
        led.switch_to_output(value=False)
        switch.switch_to_output(value=False)
        switch.value = False
        switch.switch_to_input(pull=digitalio.Pull.DOWN)

    print(protocol.CLEAR)
    clear.switch_to_output(value=False)
    clear.value = False
    clear.switch_to_input(pull=digitalio.Pull.DOWN)
    return True

def play_buzz_tone():
    buzz_tone = audiomp3.MP3Decoder(open("./buzz.mp3", "rb"))
    #audio.play(buzz_tone)
    
print("go")
buzzer_loop()
