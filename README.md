# Rusty Warrior

Simple tool to get notifications about urgent tasks from Taskwarrior in `rust`, with specific notification box.

The configuration file (to come) allows to select how many tasks you want to see, from wich report, and context.

## Prerequisites

Have your "copy" of `rust` installed.

This project has been tested only better with [dunst](https://github.com/dunst-project/dunst).

Example dunst configuration:

```
[taskwarrior]
appname = "taskwarrior"
frame_color = "#C6A13F"
foreground = "#ffffff"
background = "#170F0D"
timeout = 20
```

## Settings.toml

This file is read at runtime and can be changed before running the program. There are 3 values you can set:

- `tasks_to_show` => Number of tasks notifcations you'd like to see pop-up. [Defaults to 3]
- `exclude_project` => If there are some projects you'd like to filter out from the notification. [Defaults to ""]
- `dunst_appname` => If you've configured your [dunstrc](https://github.com/dunst-project/dunst) to something similar as above, set the `appname` from dunstrc here. [Defaults to ""]


## Installation

Simply run:

```bash
$ cargo build
```

or

```bash
$ cargo run
```

Then add the executable to you `$PATH`.


## Contributing

**Note:**
This project is released with a Contributor Code of Conduct.
By participating in this project **you agree** to abide by its terms.
See the [CODE OF CONDUCT](CODE_OF_CONDUCT.md) file for details.

1. Fork it!
2. Create your feature branch: `git checkout -b feature/name-of-feature`
3. Commit your changes: `git commit -am 'Add some feature'`
4. Push to the branch: `git push origin feature/my-new-feature`
5. Submit a pull request

## Authors

* **Chess Seventh** - Maintainer - [chess-seventh](https://github.com/chess-seventh)
* **VBerset** - Rust Guru Mentor - [vberset](https://github.com/vberset)

## License

This project is licensed under the GNU General Public License -
see the [LICENSE.md](LICENSE.md) file for details.
