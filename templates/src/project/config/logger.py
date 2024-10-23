# libraries
import os
import logging
from pathlib import Path
from logging.config import dictConfig

# modules
from {{ name }} import const


class _UserFilter(logging.Filter):
    """
    This class is for filtering log records that are user-facing.
    This is to reduce the issue of:

        logging.info("This is a message I want the user to see.")
        print("This is a message I want the user to see.")

    Where, in this case, a variable would just be wasteful.

    Instead, you can pass in: extra={'user': True} to the logger.info() call
    to return the message to the user in their terminal. This makes these calls
    much more reusable and less redundant.
    """
    def filter(self, record):
        return getattr(record, 'user', False)

def setup_logging(log_path: Path = const.LOG_FILE, log_level: str = 'INFO'):
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
            'filters': {
                'user_filter': {
                    '()': _UserFilter,
                }
            },
            'handlers': {
                'file': {
                    'class': 'logging.handlers.RotatingFileHandler',
                    'filename': log_path,
                    'maxBytes': 52428800,  # 50MB
                    'backupCount': 5,
                    'formatter': 'default',
                    'encoding': 'utf8'
                },
                'console_info': {
                    'class': 'logging.StreamHandler',
                    'formatter': 'console_default',
                    'level': 'INFO',
                    'filters': ['user_filter'], # this will only print messages with the 'user' attribute set to True

                },
                'root': {
                    'level': log_level.upper(),
                    'handlers': ['file', 'console_info'],
                },
            }
        })
        logging.getLogger(__name__)

    except Exception as e:
        print(f"Error setting up logging configuration.")
        print(e)
        raise e

if __name__ == '__main__':
    pass