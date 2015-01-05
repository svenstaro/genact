from genact.module import Module
from genact.util import draw_header
import time, random, sys

class BotnetModule(Module):
    modulename = "botnet"
    title = "Checking botnet status"

    def run(self):
        size = random.randint(100, 4000)
        connected = 0
        while connected < size:
            print("\rEstablishing connections: {}/{}".format(connected, size), end="")
            connected += 1
            time.sleep((random.random() ** 50) * 0.2)
        print()
