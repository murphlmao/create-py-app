# libraries
import os
import sys
import logging

# modules
from {{ name }} import const
from {{ name }} import filesystem
from {{ name }}.enums import ExitCode
from {{ name }}.config import configuration
from {{ name }}.logger import console_logger



def main() -> int:
    return_code: int = ExitCode.EX_ERROR.value  # default to a general error code (unbound safety)

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
        console_logger.info("This is an example of a log message that goes to the console as well as the log file")
        return_code: int = ExitCode.EX_OK.value

    except Exception as e:
        logging.exception(e)
        return_code: int = ExitCode.EX_ERROR.value


    if not (0 <= return_code <= 255):
        logging.error(f"Invalid return code {return_code}, defaulting to 255")
        return_code: int = ExitCode.EX_RANGE.value

    logging.debug(f"Exiting with code {return_code}")
    return return_code




if __name__ == "__main__":
    sys.exit(main())