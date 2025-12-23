# Protocol
The serial protocol for the buzzer microcontroller to interact with the optional computer software. We can do all this with one character if we don't care about times.

## Protocol events
### Buzz
A buzz has occurred. 
- which buzzer?

Denote by letters `A` through `J` for buzzers one through ten (A-E are green; F-J are red).
### Lockout
A player has attempted to buzz and been locked out. Unsure if this is necessary
- which buzzer?

Denote by letters `a` through `j` for buzzers one through ten (a-e are green; f-j3 are red).
### Clear
The buzzers are to be cleared. Denoted by `x`

