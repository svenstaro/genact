#!/usr/bin/env python3

import os
import random
import sys
import time


TERMINAL_SIZE = (25, 80)
if os.name == "posix":
    TERMINAL_SIZE = rows, columns = os.popen('stty size', 'r').read().split()

class ActivityGenerator(object):
    def __init__(self):
        self.activities = [self.activity_initialize, self.activity_dump_log,
                           self.activity_configure, self.activity_compile,
                           self.activity_memory_dump, self.activity_backtrace,
                           self.activity_netstat, self.activity_download]

    def do_random_activity(self):
        choice = random.choice(self.activities)
        choice()

    def draw_header(self, message, width = TERMINAL_SIZE[1],
                    centered = True, rows = 3, fill_char = "*"):
        for i in range(rows):
            if i == int(rows / 2):
                print("{0:{2}^{1}}".format(" "+message+" ", width, fill_char))
            else:
                print("{1:{1}^{0}}".format(width, fill_char))

    def activity_initialize(self):
        print("Initializing core system")

    def activity_dump_log(self):
        logs = ["kernel", "system", "user", "message", "error"]
        choice = random.choice(logs)
        if choice == "kernel":
            print("Dumping system log")

    def activity_configure(self):
        print("config")

    def activity_compile(self):
        print("compiling")

    def activity_memory_dump(self, columns = 8, format = "hex",
                             bits = 32, duration = 5, delay = 0.2):
        dumps = ["dumping memory map", "memory dump:",
                 "Dumping memory addresses", "memory image"]
        choice = random.choice(dumps)
        self.draw_header(choice)
        start_time = time.time()
        while(time.time() - start_time < duration):
            for i in range(columns):
                address = random.randint(0, pow(2, bits) - 1)
                if bits == 16:
                    print("0x%04X" % (address), end=" ")
                if bits == 32:
                    print("0x%08X" % (address), end=" ")
                if bits == 64:
                    print("0x%16X" % (address), end=" ")
                sys.stdout.flush()
                time.sleep(delay / columns)
            print()
            time.sleep(delay)

    def activity_backtrace(self):
        print("backtrace")

    def activity_netstat(self):
        print("netstat")

    def activity_download(self):
        print("download")

running = True
actgen = ActivityGenerator()
while running:
    #actgen.do_random_activity()
    actgen.activity_memory_dump()
    running = False
