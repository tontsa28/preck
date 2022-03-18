# ts3-checker-rs
This is a small tool for checking if TeamSpeak server is running. The tool can be used for such things as helper for an uptime monitor. This is a rewrite of [this project](https://github.com/tontsa28/ts3-checker).

DISCLAIMER: this tool ONLY works on UNIX-like operating systems, such as Linux distributions and MacOS (MacOS has not been tested). This does not work on NT-based operating systems like Windows because of the process detection.

## Usage
You can create your own `.env` file in which you can then store your port variable. In case you are not familiar with `.env` files, the syntax is very simple:
- `PORT=YOUR_PORT`
- For example: `PORT=3001`

You also need to set the FULL path to the pidfile, like so:
- `PIDFILE="/the/path/to/your/pidfile"`
- For example: `PIDFILE="/home/user/teamspeak-server/ts3server.pid"`
