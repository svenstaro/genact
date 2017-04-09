from genact.module import Module
from genact.util import dprint
import time, random, sys
from colorama import Fore, Style
from genact.term import *

class BotnetModule(Module):
    modulename  = "botnet"
    description = "connect to a bunch of bots and pretend to send commands"
    title       = "Checking botnet status"

    def run(self):
        clusters = [random.randint(100, 200) for i in range(random.randint(8,16))]
        onlines = [False for x in clusters]
        size = sum(clusters)

        connected = 0
        while connected <= size:
            print("\rEstablishing connections: {:4d}/{:4d}".format(connected, size), end="")
            connected += 1
            time.sleep((random.random() ** 50) * 0.05)
        print()

        time.sleep(0.3)

        for i, nodes in enumerate(clusters):
            dprint('  Cluster #{:02d} ({:3d} nodes)'.format(i, nodes))
            time.sleep(0.1)

        while True:
            print(cursor_up(len(onlines) + 1))
            for i, (nodes, online) in enumerate(zip(clusters, onlines)):
                print('{}  Cluster #{i:02d} ({nodes:3d} nodes) [{color}{s.BRIGHT}{text}{f.RESET}{s.NORMAL}]'.format(
                    erase_line(),
                    i=i,
                    nodes=nodes,
                    s=Style,
                    color=Fore.GREEN if online else Fore.YELLOW,
                    text='online' if online else 'booting',
                    f=Fore,
                ))
            if all(onlines): break
            onlines = [o or random.random() > 0.95 for o in onlines]
            time.sleep(0.1)

        for task in ["Synchronizing clocks...", "Sending login information...", "Sending command..."]:
            time.sleep(0.3)
            dprint("+ {} ".format(task), end="")
            time.sleep(0.6)
            dprint("[done]")

        dprint(">> Botnet update complete.")
