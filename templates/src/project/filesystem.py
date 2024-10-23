# libraries
import os
import logging
import configparser
from pathlib import Path
from typing import Sequence, Union

# modules


def create(directories: list, files: list):
    if directories:
        create_dirs(directories)

    if files:
        create_files(files)


def create_dirs(directories: Sequence[Union[str, Path]], verbosity: Union[dict, None] = None) -> bool:
    try:
        for directory in directories:
            # Convert string directories to Path objects
            if isinstance(directory, str):
                directory = Path(directory)

            logging.info(f"Creating directory: {directory}", extra=verbosity)
            os.makedirs(directory, exist_ok=True)

        return True

    except (PermissionError, TypeError) as e:
        logging.exception(e)
        raise

def create_files(files: Sequence[Union[str, Path]]) -> bool:
    try:
        for file in files:
            if isinstance(file, str):
                file = Path(file)

            logging.debug(f"Creating file: {file}")
            if not os.path.exists(file):
                with open(file, "w") as f:
                    pass
            else:
                logging.debug(f"File already exists: {file}")

        return True

    except (PermissionError, FileNotFoundError, TypeError) as e:
        logging.exception(e)
        raise