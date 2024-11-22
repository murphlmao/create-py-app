# libraries
import os
import logging
import configparser
from pathlib import Path

# modules
from {{ name }} import const
from {{ name }} import logger
from {{ name }} import filesystem

class Configuration:
    """
    Class for handling the configuration file for {{ name }}.
    This class creates the backbone of the application.
    """

    # OPTIONAL: Singleton pattern for configuration.
    #_instance = None
    #def __new__(cls, *args, **kwargs):
    #    """Singleton pattern so only one instance of this class can be created."""
    #    if not cls._instance:
    #        cls._instance = super(Configuration, cls).__new__(cls)
    #        cls._instance.__initialized = False
    #    return cls._instance

    def __init__(self, config_file_path: Path = const.CONFIG_FILE) -> None:
        #if self.__initialized:
        #    return
        #self.__initialized = True

        self.default_configuration = {
            'OPTIONS': {
                'LOGLEVEL': 'INFO',
            },

            # OPTIONAL: You can comment these out. This is primarily for debugging.
            "PROJECT": { # see _immutable_config_keys for why these are here
                "Name": const.APP_NAME,
                "Version": const.VERSION,
                "InstallationLocation": str(const.BASE_DIR)
            },
        }

        self.config_file_path: Path = config_file_path
        self.config = configparser.ConfigParser()
        self.config.optionxform=str # preserves case sensitivity # type: ignore

        if not os.path.exists(self.config_file_path):
            self.create_config_file()
        else:
            self.read_config_file()

    def create_logger(self):
        """Create the logger for the application."""
        config_log_level = self.config['OPTIONS']['LOGLEVEL'].upper()
        if config_log_level not in ['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']:
            logging.warning(f"Invalid logging level: {config_log_level}. Defaulting to INFO.")
            self.update_config_file('OPTIONS', 'LOGLEVEL', 'INFO')
            config_log_level = 'INFO'

        logger.setup_logging(
            log_path=const.LOG_FILE,
            log_level=config_log_level
        )

    def create_config_file(self):
        """Create configuration file if it does not exist, or add missing sections if they are missing."""
        logging.info(f"Creating configuration file at: {self.config_file_path}")

        os.makedirs(self.config_file_path.parent, exist_ok=True)
        self._write_std_config_defaults()

        self.configuration_data = self.read_config_file(validate=True)

    def _write_std_config_defaults(self) -> bool:
        """
        Writes the default configuration into
        the configuration file based on the
        default_configuration attribute.
        """
        try:
            for section, keys in self.default_configuration.items():
                self.config[section] = keys

            with open(self.config_file_path, 'w') as config_file:
                self.config.write(config_file)

            logging.info(f"Default configuration written to {self.config_file_path}")
            return True

        except Exception as e:
            logging.error(f"Failed to write configuration file")
            logging.exception(e)
            return False

    def read_config_file(self, validate: bool=True):
        """Read the configuration file & return the configuration data."""
        self.config.read(self.config_file_path)
        logging.info(f"Configuration file read from: {self.config_file_path}")

        if validate:
            if not self.validate_configuration_file():
                logging.warning("Configuration file validation failed.")
                self.repair_configuration_file()

        return self.config

    def validate_configuration_file(self) -> bool:
        """Validate the configuration file for required keys & sections."""
        for section, keys in self.default_configuration.items():
            if section not in self.config:
                logging.warning(f"Missing section: {section}")
                return False

            for key in keys:
                if key not in self.config[section]:
                    logging.warning(f"Missing key: {key} in section: {section}")
                    return False

        self._immutable_config_keys()
        return True

    def repair_configuration_file(self) -> None:
        """Repair the configuration file by adding missing sections or keys."""
        for section, keys in self.default_configuration.items():
            if section not in self.config:
                self.config[section] = {}

            for key, value in keys.items():
                if key not in self.config[section]:
                    self.config[section][key] = value
                    logging.info(f"Repaired missing key: {key} in section: {section} with default value")

        with open(self.config_file_path, 'w') as config_file:
            self.config.write(config_file)

        logging.info(f"Configuration file repaired at: {self.config_file_path}")

    def _immutable_config_keys(self):
        """Make specific config keys immutable. This is due to project specific info that should not be changed."""
        logging.debug("Setting immutable keys.")
        self.update_config_file('PROJECT', 'Name', const.APP_NAME)
        self.update_config_file('PROJECT', 'Version', const.VERSION)
        self.update_config_file('PROJECT', 'InstallationLocation', const.BASE_DIR)

        current_log_level = self.config['OPTIONS']['LOGLEVEL']
        self.update_config_file('OPTIONS', 'LoggingLevel', current_log_level.upper())

    def update_config_file(self, section: str, option: str, value):
        """
        Update the configuration file with a new value for a specific option.
        """
        if section not in self.config:
            self.config[section] = {}

        self.config[section][option] = str(value)

        with open(self.config_file_path, 'w') as config_file:
            self.config.write(config_file)


# Example usage
if __name__ == "__main__":
    config_path = const.CONFIG_FILE

    config = Configuration(config_path)
    config.read_config_file()  # Load or create config file
    config.update_config_file('OPTIONS', 'LOGLEVEL', 'DEBUG')


    filesystem.create(
        directories=const.STD_DIRECTORIES_TO_CREATE,
        files=const.STD_FILES_TO_CREATE
    )

    x = config.read_config_file()  # Load or create config file
    print(x.sections())

    new_config = Configuration(config_path)
    y = new_config.read_config_file()  # Load or create config file
    print(y.sections())

    if x == y:
        print("Singleton pattern working")