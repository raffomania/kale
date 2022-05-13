# Kale

A small TUI calendar app.

- ⚡ Fast
- ⌨ Keyboard-controllable
- 📊 Google-calendar-like weekly overview
- 🔌 Offline-first, using [vdirsyncer](https://github.com/pimutils/vdirsyncer) 

## Status

Kale can show non-recurring events from a hard-coded directory of caldav files. I'd say it's not yet ready for any real use.

## Alternative Projects

I've compiled a short list containing my highly subjective reasons to reinvent the wheel instead of using existing software:

- [*GNOME Calendar*](https://gitlab.gnome.org/GNOME/gnome-calendar) has a nice GTK interface but requires 200MB of GNOME dependencies and takes longer than a minute to start on my machine.
- [*khal*](https://github.com/pimutils/khal) offers a traditional CLI and a TUI, but lacks the graphic weekly overview of kale.
- [*Lightning*](https://www.thunderbird.net/en-US/calendar/) Couldn't get this one to work.
- [*gcalcli*](https://github.com/insanum/gcalcli) Has a nice weekly overview, but only works with google calendar.
- [*calendar-cli*](https://github.com/tobixen/calendar-cli) Doesn't work offline.
- [*Nextcloud Calendar*](https://github.com/nextcloud/calendar) Limited keyboard control, Web-based, doesn't work offline.
- [*BORG Calendar*](https://github.com/mikeberger/borg_calendar) Doesn't support DAV syncing.
- [*KOrganizer*](https://invent.kde.org/pim/korganizer) Limited keyboard control.
- [*calcurse*](https://github.com/lfos/calcurse) No graphical weekly overview, experimental DAV syncing.
