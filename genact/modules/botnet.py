from genact.module import Module
from genact.util import dprint
import time, random, sys
from colorama import Fore, Style

class BotnetModule(Module):
    modulename  = "botnet"
    description = "connect to a bunch of bots and pretend to send commands"
    title       = "Checking botnet status"

    def run(self):
        clusters = [random.randint(100, 200) for i in range(random.randint(8,16))]
        size = sum(clusters)

        connected = 0
        while connected < size:
            print("\rEstablishing connections: {:4d}/{:4d}".format(connected, size), end="")
            connected += 1
            time.sleep((random.random() ** 50) * 0.1)
        print()

        for i, cluster in enumerate(clusters):
            dprint("  Cluster #{:02d} ({:3d} nodes) -- ".format(i, cluster), end="")
            time.sleep(random.random() * 0.8)
            print("[{f.GREEN}{s.BRIGHT}Online{f.RESET}{s.NORMAL}]".format(f=Fore, s=Style))
            time.sleep(random.random() * 0.4)

        for task in ["Synchronizing clocks...", "Sending login information...", "Sending command..."]:
            time.sleep(0.3)
            dprint("+ {} ".format(task), end="")
            time.sleep(0.6)
            dprint("[done]")

        dprint(">> Botnet update complete.")
