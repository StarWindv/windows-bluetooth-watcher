from importlib.metadata import version

from windows_bluetooth_watcher.windows_bluetooth_watcher import *


__author__ = "星灿长风v(StarWindv)"
__license__ = "GPL-3.0-only"
__home__ = "https://github.com/starwindv/windows-bluetooth-watcher.git"
__description__ = """\
This project is secondary development based on the `windows crate`,\
primarily wrapping related methods for obtaining desktop toast notifications on Windows systems,\
and uses PyO3 for Python bindings to provide it as a Python library.
"""
__version__ = version(__name__)


__all__ = windows_bluetooth_watcher.__all__

# clean up
# noinspection PyUnresolvedReferences
del windows_bluetooth_watcher
del version
