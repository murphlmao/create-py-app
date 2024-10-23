# libraries
import os
import sys
import logging
import configparser

# modules
from {{ name }} import const
from {{ name }}.import filesystem
from {{ name }}.config import configuration



def main():
    filesystem.create(
        directories=const.STD_DIRECTORIES_TO_CREATE,
        files=const.STD_FILES_TO_CREATE
    )

    # main logic
    config = configuration.Configuration(dev_mode=const.DEV_MODE)
    config_data: configparser.ConfigParser = config.read_config_file()




if __name__ == "__main__":
    sys.exit(main())