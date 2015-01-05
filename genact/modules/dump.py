from genact.module import Module
from genact.util import draw_header
import time, random, sys

dumps = ["dumping memory map", "memory dump:", "Dumping memory addresses", "memory image"]
columns = 8
format = "hex"
bits = 32
duration = 5
delay = 0.2

class DumpModule(Module):
    __modulename__ = "dump"

    def run(self):
        draw_header(random.choice(dumps))

        start_time = time.time()
        while(time.time() - start_time < duration):
            for i in range(columns):
                address = random.randint(0, pow(2, bits) - 1)
                print("{0:0<#{1}{2}}".format(address, int((bits / 4) + 2), "x"), end=" ")
                sys.stdout.flush()
                time.sleep(delay / columns)
            print()
            time.sleep(delay)
