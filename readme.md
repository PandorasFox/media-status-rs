# media-status

(this is my first rust project, please be gentle)

run via `~/.cargo/bin/media-status spotify` (replace the path/player name as necessary, etc and so forth). 

preliminary support for [mpris-integration](https://github.com/jsmnbom/mpris-integration/) in firefox.

* Only handles title, album, and artist.
* Uses dbus PropertiesChanged to efficiently wait for property updates
* Could probably handle progress-percentage stuff if spotify exported that :(
* Output is json, for [Waybar](https://github.com/Alexays/waybar)
* install with `cargo install --git https://github.com/pandorasfox/media-status.git`
  * If you're curious about the timing (in microseconds) for how long certain parts of the code takes, install with `--features timing` appended
