# ts3-checker-rs
This is a small tool for checking if TeamSpeak server is running. The tool can be used for such things as helper for an uptime monitor. This is an improved Rust rewrite of [this project](https://github.com/tontsa28/ts3-checker).

DISCLAIMER: this tool ONLY works on UNIX-like operating systems, such as Linux distributions and MacOS (MacOS has not been tested). This does not work on NT-based operating systems like Windows because of the process detection.

If you want to get any CPU architecture supported in the precompiled binaries, feel free to open an issue and I'll take a look into it. Currently, `x86_64` and `aarch64` are supported, but the program should compile fine on other architectures as well.

One more thing: when downloading the precompiled binaries, you might need to use this command `chmod +x binary_file`. This is required because at least the CI does not give execution permissions for the program.

## Usage
NOTE: `.env` file must be present at runtime. `Rocket.toml` must be specified if the default settings need to be modified. Both files must exists in the same path.
You can create your own `.env` file in which you can then store your `PIDFILE` variable. In case you are not familiar with `.env` files, the syntax is very simple:
- `PIDFILE="/the/path/to/your/pidfile"`
- For example: `PIDFILE="/home/user/teamspeak-server/ts3server.pid"`

NOTE: You also need to create Rocket.toml file for configuring Rocket. You need at least the following parameters: `[config]`, `address` and `port`.
- Example Rocket.toml file:\
`[default]`\
`address = "127.0.0.1"`\
`port = 3001`
