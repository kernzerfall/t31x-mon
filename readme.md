# t31x-mon

A quick 'n' dirty thrown together utility to display the temperature from
TP-Link Tapo T31x sensors on a statusbar (e.g. waybar/eww/polybar).

This requires a secret service daemon to be running, so that it can
save/retrieve your Tapo password to/from your keyring.

For usage, look at `t31x-mon --help`.

You can run the utility with `-l DEBUG` to get a list of available
`device_id`s.

The symbols referenced in unicode rely on NerdFonts.
