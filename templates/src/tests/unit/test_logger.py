# libraries
import os
import pytest
import logging
from pathlib import Path
from unittest.mock import patch, MagicMock
from logging.handlers import RotatingFileHandler

# modules
from {{ name }} import const
from {{ name }}.logger import setup_logging, console_logger


@pytest.fixture
def temp_log_file(tmp_path):
    """Fixture to provide a temporary log file path."""
    log_dir = tmp_path / "logs"
    log_dir.mkdir()
    log_file = log_dir / "test.log"
    return log_file


def test_setup_logging_creates_directories_and_files(temp_log_file):
    """Test that setup_logging creates log directories and files."""
    assert not temp_log_file.exists()  # File shouldn't exist before setup

    setup_logging(log_path=temp_log_file)
    assert temp_log_file.exists()  # File should exist after setup
    assert temp_log_file.parent.exists()  # Directory should exist


def test_invalid_log_level_defaults_to_info(temp_log_file, caplog):
    """Test that invalid log levels default to INFO."""
    setup_logging(log_path=temp_log_file, log_level="INVALID")

    logger = logging.getLogger()
    logger.warning("Test warning")
    logger.info("Test info")

    with temp_log_file.open() as log_file:
        logs = log_file.read()
        assert "WARNING - Test warning" in logs
        assert "INFO - Test info" in logs
        assert "Invalid logging level" in caplog.text


def test_console_logger_logs_to_console_and_file(temp_log_file, capsys):
    """Test that console_logger logs to both the console and file."""
    setup_logging(log_path=temp_log_file)

    console_logger.info("Console log test")
    captured = capsys.readouterr()

    assert "Console log test" in captured.out  # Console output
    with temp_log_file.open() as log_file:
        logs = log_file.read()
        assert "Console log test" in logs  # File output


def test_log_rotation(temp_log_file):
    """Test that log rotation works as expected."""
    # Temporarily set maxBytes to a small value for testing
    max_bytes = 1024  # 1KB
    setup_logging(log_path=temp_log_file, log_level="DEBUG")

    # Modify the handler's maxBytes directly for the test
    handler = next(
        h for h in logging.getLogger().handlers if isinstance(h, RotatingFileHandler)
    )
    handler.maxBytes = max_bytes

    # Write enough logs to exceed maxBytes and trigger rotation
    for _ in range(100):
        logging.getLogger().debug("This is a debug log" * 100)

    rotated_logs = list(temp_log_file.parent.glob("test.log.*"))
    assert len(rotated_logs) > 0  # Ensure rotation created backup files

def test_log_format(temp_log_file):
    """Test that log messages follow the defined format."""
    setup_logging(log_path=temp_log_file, log_level="DEBUG")

    logging.debug("Debug message")
    with temp_log_file.open() as log_file:
        logs = log_file.read()
        assert "DEBUG" in logs
        assert "Debug message" in logs
        assert ":" in logs  # Check for line number formatting