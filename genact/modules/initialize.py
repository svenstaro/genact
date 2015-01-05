from genact.util import draw_progress
from genact.module import Module

class InitializeModule(Module):
    __modulename__ = "initialize"

    def run(self):
        draw_progress("Initializing core system", "done")
