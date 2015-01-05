#!/usr/bin/env python3

import genact.modules
import os, random, argparse
from genact.module import all_modules

# Parse arguments from command line
parser = argparse.ArgumentParser(description="A nonsense activity generator")
parser.add_argument('-m', '--modules', metavar='M', nargs='+', choices=all_modules.keys(),
                    help='Provide one or more of: ' + ', '.join(all_modules.keys()))
args = parser.parse_args()

# Run active modules
active_modules = list(all_modules.values())
if args.modules:
    active_modules = [m for m in active_modules if m.__modulename__ in args.modules]

while True:
    try:
        module = random.choice(active_modules)
        module().run()
    except KeyboardInterrupt:
        print()
        print("Finished.")
        break
