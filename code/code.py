import digitalio
import board
import audiomp3
import audiopwmio
CLEAR_PIN = board.GP10
# (led, switch)
RED_TEAM_PINS = [(board.GP17, board.GP16)]
GREEN_TEAM_PINS = []

clear = digitalio.DigitalInOut(CLEAR_PIN)
clear.switch_to_input(pull=digitalio.Pull.DOWN)

red_pins = [(digitalio.DigitalInOut(x), digitalio.DigitalInOut(y)) for x,y in RED_TEAM_PINS]
green_pins = [(digitalio.DigitalInOut(x), digitalio.DigitalInOut(y)) for x,y in GREEN_TEAM_PINS]
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
        play_buzz_tone() # todo: different tones for different teams
       
        await_clear()
        led_pin.value = False

def await_buzz(red, green):
    while True:
        for led, switch in red + green:
            if switch.value == True:
                return led
def await_clear():
    while clear.value == False:
        if not audio.playing:
            audio.stop()
    #time.sleep(0.001)
    #if clear.value == True:
    #    time.sleep(0.001)
    #    await_clear()
    print("clearing")
    clear.switch_to_output(value=False)
    clear.value = False
    clear.switch_to_input(pull=digitalio.Pull.DOWN)
    return True

def play_buzz_tone():
    buzz_tone = audiomp3.MP3Decoder(open("./buzz.mp3", "rb"))
    audio.play(buzz_tone)

    
print("go")
buzzer_loop()
