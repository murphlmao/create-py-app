# libraries
import sys
import logging
import configparser

# modules
from {{ name }}.config import configuration
from {{ name }} import const



def main():
    # main logic
    config = configuration.Configuration(dev_mode=const.DEV_MODE)
    config_data: configparser.ConfigParser = config.read_config_file()




if __name__ == "__main__":
    sys.exit(main())

-