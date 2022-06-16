# pid-checker
This is a small tool for checking if a pid file exists. The tool can be used for such things as helper for an uptime monitor. This is an improved Rust rewrite of [this project](https://github.com/tontsa28/ts3-checker).

DISCLAIMER: this tool ONLY works on UNIX-like operating systems, such as Linux distributions and macOS (macOS has not been tested). This does not work on NT-based operating systems like Windows because of the process detection.

If you want to get any CPU architecture supported in the precompiled binaries, feel free to open an issue, and I'll take a look into it. Currently, `x86_64` and `aarch64` are supported, but the program should compile fine on other architectures as well.

One more thing: when downloading the precompiled binaries, you might need to use this command `chmod +x binary_file`. This is required because the CI does not give execution permissions for the program.

## Usage
NOTE: a `.env` file must be present in the same directory along with the program.
This file must include three variables: `PID_FILE`, `ADDRESS` and `PORT`. `PID_FILE` and `ADDRESS` have to be strings, while `PORT` must be an integer.
For debugging, it is recommended to have `ADDRESS` value set to `"127.0.0.1"` (localhost).
In production, you most likely want to open the port to the internet and use `"0.0.0.0"` instead.
The `PORT` value can be any value from 0 to 65535 (as long as the port is not being used by another program).

You can create your own `.env` file in which you can then store your environment variables. In case you are not familiar with `.env` files, the syntax is very simple:
- `PID_FILE="/the/path/to/your/pidfile"`
- For example: `PID_FILE="/home/user/teamspeak-server/ts3server.pid"`

An example of a `.env` file:\
`PID_FILE="/home/user/teamspeak-server/ts3server.pid"`\
`ADDRESS="127.0.0.1"`\
`PORT=3001`