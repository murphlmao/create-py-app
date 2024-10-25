# libraries
import os
import sys
import platform
from pathlib import Path
from typing import Final, List, Dict

# modules
from {{ name }} import metadata


def _get_installation_location() -> Path:
    """
    Quick function to get the installation
    location of the script. Will detect if
    the app is a binary (frozen) or not.
    """
    return Path(os.path.dirname(sys.executable) if getattr(sys, 'frozen', False) else os.path.dirname(__file__))

# determining system & runtime context
__installation_location__: Path = _get_installation_location()
_platform: str = platform.system().lower()
IS_WINDOWS: Final[bool] = _platform == 'windows'
IS_UNIX: Final[bool] = _platform in ('linux', 'darwin')

BASE_DIR: Path = Path()
WINDOWS_DEFAULT_PATH: Final[Path] = Path("C:/")
LINUX_DEFAULT_PATH: Final[Path] = Path("/tmp/")


DEV_MODE: Final[bool] = os.getenv('DEV', 'false').lower() == 'true'
BASE_DIR = Path(WINDOWS_DEFAULT_PATH if IS_WINDOWS else LINUX_DEFAULT_PATH)

if DEV_MODE: # assume production mode
    BASE_DIR = _get_installation_location() / "_" # ./src/{{ name }}/_/


### {{ name }} Constants ###
APP_NAME: Final[str] = metadata.__title__
BINARY_NAME: Final[str] = APP_NAME + ('.exe' if IS_WINDOWS else '')
VERSION: Final[str] = metadata.__version__
INIT_TITLE: Final[str] = metadata.__initialization_title__
DESCRIPTION: Final[str] = metadata.__description__

# general filenames
CONFIG_FILENAME: Final[str] = "config.ini"
LOG_FILENAME: Final[str] = f"{APP_NAME}.log"

# base paths for {{ name }}
CONFIG_DIR: Final[Path] = BASE_DIR / "config"
LOGS_DIR: Final[Path] = BASE_DIR / "logs"

# full paths of necessary files
CONFIG_FILE: Final[Path] = CONFIG_DIR / CONFIG_FILENAME
LOG_FILE: Final[Path] = LOGS_DIR / LOG_FILENAME


# misc constants
LOG_TO_CONSOLE: Final[Dict[str, bool]] = {"user": True} # see ./config/logger.py/_UserFilter for more info


### standard directories and files to create
# if these directories/files already exist, it will *not* overwrite them
# nor change their configuration. They just need to exist for the application to run.

STD_DIRECTORIES_TO_CREATE: List[Path] = [
    CONFIG_DIR,
    LOGS_DIR,
]

STD_FILES_TO_CREATE: List[Path] = [
    # const.CONFIG_FILE gets created during configuration initialization
    # const.LOG_FILE gets created during logging initialization
]


if __name__ == "__main__":
    # debugging: print all of these variables
    for var in dir():
        if var.isupper():
            print(var, "=", eval(var))