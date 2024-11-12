# libraries
import os
import sys
import logging

# modules
from {{ name }} import const
from {{ name }} import filesystem
from {{ name }}.config import configuration



def main() -> int:
    try:
        filesystem.create(
            directories=const.STD_DIRECTORIES_TO_CREATE,
            files=const.STD_FILES_TO_CREATE
        )

        # main logic
        config = configuration.Configuration()
        if not os.path.exists(config.config_file_path):
            config.create_config_file()
        else:
            config.read_config_file(validate=True)

        config.create_logger()
        logging.info(f"Starting up {const.APP_NAME}, version: {const.VERSION}")

        return 0

    except Exception as e:
        logging.exception(e)
        return 1





if __name__ == "__main__":
    sys.exit(main())