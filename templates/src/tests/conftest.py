# conftest.py - configuration file for pytest
import os
import sys

# Ensure pytest is run from the 'src' directory to expose our modules
sys.path.append(os.path.join(os.path.dirname(__file__), '..', ''))

