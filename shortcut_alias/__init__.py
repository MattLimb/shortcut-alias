from sys import path
import yaml
import pathlib

from jinja2 import Environment

__author__ = "Matt Limb <matt.limb17@gmail.com>"

SETTINGS = {
    "show_command": True,
    "show_reason": True,
    "show_output": True,
    "show_output_header": True,
    "show_skip": True,
    "colour": True
}

GLOBAL_TEMPLATE_ENVIRONMENT = Environment()

def load_settings(filepath):
    filepath = pathlib.Path(filepath)
    
    if filepath.exists():
        with filepath.open("r") as f:
            tmp_set = yaml.safe_load(f.read())
        
        SETTINGS.update(convert_all_sets(tmp_set))

    else:
        with filepath.open("w") as f:
            f.write(
                yaml.safe_dump(SETTINGS, indent=2)
            )

def setup_settings(filepath):
    root = pathlib.Path(filepath)
    config = pathlib.Path(root, "shortcut.d")

    root.mkdir()
    config.mkdir()

    with pathlib.Path(root, "settings.yaml").open("w") as f:
        f.write(
            yaml.safe_dump(SETTINGS, indent=2)
        )

def convert_all_sets(settings):
    # Get all setting
    all_settings = dict(**SETTINGS)
    all_set = settings.get("show_all")
    n_val = True

    if all_set != None:
        if all_set == True:
            n_val = True
        else:
            n_val = False

        all_settings = { 
            key: n_val 
            for key, _ in all_settings.items()
        }

        settings.update(all_settings)
        del settings["show_all"]
    
    return settings