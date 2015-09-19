from genact.module import Module
import time, random, sys
from genact.util import dprint

titles = ["Dumping memory map", "Memory dump", "Dumping memory addresses", "Memory image"]
columns = 8
format = "hex"
bits = 32
duration = 5
delay = 0.4

class DumpModule(Module):
    modulename  = "dump"
    description = "dump random registers of memory in hex format"

    @property
    def title(self):
        return random.choice(titles)

    def run(self):
        for i in range(random.randint(4, 6)):
            data = [random.randint(0, pow(2, bits) - 1) for i in range(columns)]
            # data = [i*3 for i in range(columns)]
            data = map(lambda n: "{:0>#10x}".format(n), data)
            data = " ".join(data)
            dprint(data, chunksize=11, delay=0.1)
            time.sleep(0.5)
