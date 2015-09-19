from genact.module import Module
from genact.util import human_readable_filesize
import time, random, sys

width = 60
delay = 0.2
ports = [21, 22, 80, 81, 443, 25652, 1337]

class DownloadModule(Module):
    modulename  = "download"
    description = "progress bar for downloading a file from the internet"
    title       = "Download manager"

    def run(self):
        # Generate random download figures
        ip = ".".join([str(random.randint(start, 255)) for start in (0, 1, 1, 0)])
        port = str(random.choice(ports))
        file_size = random.randint(100000, 1000000)
        download_speed = random.randint(6000, 200000)

        # Print Information
        print("Connecting to {0} on port {1} ...".format(ip,port));
        print("File size: {0}".format(human_readable_filesize(file_size)))

        received_size = 0 # bytes per second

        # Start the download
        while True:
            if received_size > file_size:
                received_size = file_size

            speed = download_speed * (1 + random.randint(-1000, 1000) / 10000.0)
            i = int(received_size / file_size * width)

            print("\r{0: >6.1%} [{2: <{3}}] {1: >10}/s".format(
                received_size / file_size,
                human_readable_filesize(speed),
                "=" * i, width), end="")

            sys.stdout.flush()

            if received_size >= file_size:
                break

            received_size += speed * delay
            time.sleep(delay)

        print()
        print("Download finished.")
