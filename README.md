A quick hack to be able to practice accordion while travelling. It runs in the terminal and creates a virtual MIDI-instrument that you can use in any MIDI program, like Garageband.

This is very hard coded for my situation: a US layout keyboard and a type B chromatic button system.

The terminal is not well suited for detecting key releases. It's tested and working in the [Kitty terminal](https://sw.kovidgoyal.net/kitty/).

### Sample run

```
$ cargo run -q
Use your keyboard as an accordion!
Releasing keys only works in some terminals, like kitty.
Press ESC to quit.
Playing C
Playing D#
Playing F#
Releasing D#
Releasing F#
Releasing C
```
