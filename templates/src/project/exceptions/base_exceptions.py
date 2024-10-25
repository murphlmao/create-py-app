class GenericException(Exception):
    def __init__(self, message: str, status_code: int = 1):
        """Generic exception class for the project
        Message: str: Message to return in the response
        status_code: int: Status code to return in the response
            Generally, try to follow these guidelines:
            - https://www.gnu.org/software/libc/manual/html_node/Exit-Status.html
            - https://tldp.org/LDP/abs/html/exitcodes.html\
            - https://man.freebsd.org/cgi/man.cgi?query=sysexits
        """
        self.message = message
        self.status_code = status_code
        super().__init__(self.message)

    def to_dict(self):
        return {"message": self.message}