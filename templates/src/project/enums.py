# libraries
from enum import IntEnum

__all__ = ['ExitCode']


class ExitCode(IntEnum):
    """Exit codes for the application

    Generally, try to follow these guidelines:
    - https://www.gnu.org/software/libc/manual/html_node/Exit-Status.html
    - https://tldp.org/LDP/abs/html/exitcodes.html
    - https://man.freebsd.org/cgi/man.cgi?query=sysexits
    """
    EX_OK = 0                # Successful termination
    EX_ERROR = 1             # General error
    EX_MISUSE = 2            # Incorrect usage or invalid arguments
    EX_USAGE = 64            # Command line usage error
    EX_DATAERR = 65          # Data format error
    EX_NOINPUT = 66          # Cannot open input
    EX_NOUSER = 67           # Addressee unknown
    EX_NOHOST = 68           # Hostname unknown
    EX_UNAVAILABLE = 69      # Service unavailable
    EX_SOFTWARE = 70         # Internal software error
    EX_OSERR = 71            # System error (e.g., can't fork)
    EX_OSFILE = 72           # Critical OS file missing
    EX_CANTCREAT = 73        # Can't create (user) output file
    EX_IOERR = 74            # Input/output error
    EX_TEMPFAIL = 75         # Temporary failure; retry later
    EX_PROTOCOL = 76         # Remote protocol error
    EX_NOPERM = 77           # Permission denied
    EX_CONFIG = 78           # Something was found in an unconfigured or misconfigured state.
    EX_TERM = 130            # User terminated the program with Control-C
    EX_RANGE = 255           # Exit status out of range