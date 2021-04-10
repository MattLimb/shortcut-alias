
import argparse
import os
import pathlib
from colorama import init
from . import SETTINGS, load_settings, setup_settings

from .shortcut import Shortcut

__author__ = "Matt Limb <matt.limb17@gmail.com>"

def main():
    # Find Home Directory
    home = pathlib.Path(pathlib.Path.home(), "shortcut")

    # Setup Setting Path
    setting_config = pathlib.Path(os.environ.get("SHORTCUT_CONFIG", str(home)))

    # Setup Config if it doesn't exist
    if not setting_config.exists():
        setup_settings(setting_config)

    # Load settings from file
    load_settings(pathlib.Path(setting_config, "settings.yaml"))

    # Setup colour is enabled
    if SETTINGS["colour"]:
        init()
    
    # Setup Command Dict
    commands = dict()

    # Setup the commands
    for config in pathlib.Path(setting_config, "shortcut.d").glob("*.yaml"):
        s = Shortcut.from_file(config)

        for x in s:
            commands[x.cmd] = x    
        
    # Setup CLI parser
    parser = argparse.ArgumentParser(prog="shortcut")
    sub_parser = parser.add_subparsers(help="Commands", dest="command")

    # Register commands with CLI
    for _, cmd in commands.items():
        cmd.add_parser(sub_parser)

    # Parse CLI    
    args = parser.parse_args()

    if args.command == None:
        args = parser.parse_args([ "--help" ])

    # Run the Command
    commands[args.command].run_commands(args)

if __name__ == "__main__":
    main()