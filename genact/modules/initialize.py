from genact.util import draw_progress, draw_header, lerp
from genact.module import Module
from random import random

class InitializeModule(Module):
    modulename = "initialize"
    title = "Initializing core system"

    def run(self):
        draw_progress("Loading libraries", "done", duration=lerp(random(), 1.0, 2.0))
        draw_progress("Restoring state  ", "done", duration=lerp(random(), 0.3, 0.4))
        draw_progress("Initializing core", "done", duration=lerp(random(), 0.4, 0.6))
        draw_progress("Loading plugins  ", "done", duration=lerp(random(), 0.7, 1.0))
        draw_progress("Finalizing report", "done", duration=lerp(random(), 0.3, 0.4))
        draw_progress("Cleaning up      ", "done", duration=lerp(random(), 0.6, 1.2))
