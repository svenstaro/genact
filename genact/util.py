import os, random, time, sys
from colorama import Fore, Back, Style

# Read terminal size
TERMINAL_SIZE = (25, 80)
if os.name == "posix":
    TERMINAL_SIZE = rows, columns = os.popen('stty size', 'r').read().split()

def dprint(s, delay=0.01, chunksize=1, **kwargs):
    for i in range(0, len(s), chunksize):
        print(s[i:i+chunksize], end="")
        sys.stdout.flush()
        time.sleep(delay)
    print(**kwargs)

def lerp(t, a, b):
    return t * (b - a) + a

def human_readable_filesize(size):
    names = ['KiB', 'MiB', 'GiB', 'TiB', 'PiB', 'EiB', 'ZiB', 'YiB']
    for name in names:
        size /= 1024
        if size < 1024:
            return '{0:.1f} {1}'.format(size, name)
    return "huge!"

def draw_header(message):
    print(">> {s.BRIGHT}{f.WHITE}{message}{s.NORMAL}{f.RESET}".format(f=Fore, s=Style, message=message))

def draw_progress(left_side, right_side, width=30, duration=5, fill_char="="):
    start_time = time.time()
    spinner_statuses = ["|", "/", "-", "\\"]
    spinner_pos = 0
    elapsed_time = time.time() - start_time
    while(elapsed_time < duration):
        elapsed_time = time.time() - start_time

        # Make sure to get a fill_char at the end of our cycle when done.
        if elapsed_time >= duration:
            spinner_statuses = [fill_char]
            spinner_pos = 0

        print("\r{3} [{0:{0}^{1}}{2}]".format(fill_char,
                                    int((width / duration) * elapsed_time),
                                    spinner_statuses[spinner_pos],
                                    left_side, right_side), end="")
        sys.stdout.flush()

        if spinner_pos >= len(spinner_statuses) - 1:
            spinner_pos = 0
        else:
            spinner_pos += 1

        time.sleep(duration / width)
    print()
