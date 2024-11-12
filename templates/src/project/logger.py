# libraries
import os
import sys
import logging
from pathlib import Path
from logging import Logger
from logging.config import dictConfig

# modules
from {{ name }} import const


def setup_logging(log_path: Path = const.LOG_FILE, log_level: str = 'INFO'):
    if log_level.upper() not in ['DEBUG', 'INFO', 'WARNING', 'ERROR', 'CRITICAL']:
        logging.warning(f"Invalid logging level: {log_level}. Defaulting to INFO. Please check your configuration file.")
        log_level = 'INFO'

    if not os.path.exists(log_path.parent):
        os.makedirs(log_path.parent, exist_ok=True)

    try:
        dictConfig({
            'version': 1,
            'formatters': {
                'default': {
                    'format': '%(asctime)s %(module)s:%(funcName)s:%(lineno)d: %(levelname)s - %(message)s',

                },
                'console_default': {
                    'format': '%(message)s',
                },
            },

            'handlers': {
                'file': {
                    'class': 'logging.handlers.RotatingFileHandler',
                    'filename': log_path,
                    'maxBytes': 52428800,  # 50MB
                    'backupCount': 5,
                    'formatter': 'default',
                    'encoding': 'utf8',
                    'level': log_level.upper(),
                },
                'console_info': {
                    'class': 'logging.StreamHandler',
                    'formatter': 'console_default',
                    'level': 'INFO',
                },
                # handler for user_logger that always prints to the terminal
                'user_console': {
                    'class': 'logging.StreamHandler',
                    'formatter': 'console_default',
                    'level': 'INFO',
                    'stream': sys.stdout,
                },
            },
            'root': {
                'level': log_level.upper(),
                'handlers': ['file', 'console_info'],
            },
            'loggers': {
                # independent logger, not dependent on root
                'user_logger': {
                    'level': 'INFO',
                    'handlers': ['user_console', 'file'],
                    'propagate': False,  # ensure it doesn't propagate to root
                }
            }
        })


    except Exception as e:
        print("Error Creating Logs Path")
        print(e)
        raise e

# always executed on import
logging.getLogger(__name__)
user_logger: Logger = logging.getLogger('user_logger')


if __name__ == '__main__':
    pass