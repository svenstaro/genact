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
                print("{0:{0}^{1}}".format(fill_char, width))

    def draw_progress(self, width = 30, duration = 5, fill_char = "="):
        start_time = time.time()
        spinner_statuses = ["|", "/", "-", "\\"]
        spinner_pos = 0
        time_delta = time.time() - start_time
        while(time_delta < duration):
            time_delta = time.time() - start_time
            print("\r{0:{0}^{1}}{2}".format(fill_char,
                                        int((width / duration) * time_delta),
                                        spinner_statuses[spinner_pos]), end="")
            if spinner_pos >= len(spinner_statuses) - 1:
                spinner_pos = 0
            else:
                spinner_pos += 1
            sys.stdout.flush()
            time.sleep(duration / width)

    def activity_initialize(self):
        print("{0: <30}".format("Initializing core system"), end="")
        self.draw_progress()
        print()

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
                print("{0:0<#{1}{2}}".format(address, int((bits / 4) + 2), "x"),
                                             end=" ")
                sys.stdout.flush()
                time.sleep(delay / columns)
            print()
            time.sleep(delay)

    def activity_backtrace(self):
        print("backtrace")

    def activity_netstat(self):
        print("netstat")

    def activity_download(self):
        download_progress = 0
        file_size = random.randint(100000, 1000000)
        ip = str(random.randint(1,255)) + "." + str(random.randint(1,255)) + "." + str(random.randint(1,255)) + "." + str(random.randint(1,255))
        port = str(random.choice([21,22,80,443,25652,1337]))
        print("Connecting to " + ip + " on port " + port + " ...");
        print("File size: {}".
                format(ActivityGenerator.human_readable_size(file_size)))
        download_speed = random.randint(6000, 200000) # bytes per second
        received_size = 0
        delay = 0.2
        width = 40
        done = False
        while not done:
            if received_size > file_size:
                received_size = file_size
            speed = download_speed * (1 + random.randint(-1000,1000) / 10000.0)
            i = int(received_size / file_size * width)
            print("\r{0: >6.1%} [{2: <40}] {1: >10}/s".format(
                received_size / file_size,
                ActivityGenerator.human_readable_size(speed),
                "=" * i), end="")
            sys.stdout.flush()
            if received_size >= file_size:
                done = True
            received_size += speed * delay
            time.sleep(delay)
        print("\nDownload finished.")

    def human_readable_size(size):
        names = ['KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']
        for name in names:
            size /= 1024
            if size < 1024:
                return '{0:.1f} {1}'.format(size, name)
        return "huge!"

running = True
actgen = ActivityGenerator()
while running:
    #actgen.do_random_activity()
    actgen.activity_download()
    running = False
