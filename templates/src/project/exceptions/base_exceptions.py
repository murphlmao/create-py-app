class GenericException(Exception):
    """Generic exception class for the project

    Attributes:
        message: Explanation of the error.
    """
    def __init__(self, message: str):
        super().__init__(message)