#!/usr/bin/env python3

import genact.modules
import os, random, argparse, sys
from genact.util import draw_header
from genact.module import all_modules
from colorama import Fore, Back, Style

# Parse arguments from command line
parser = argparse.ArgumentParser(description="A nonsense activity generator")
parser.add_argument('-m', '--modules', nargs='+', help='Select which modules to run')
parser.add_argument('-H', '--no-headers', action='store_const', const=True, help='Hide headers')
parser.add_argument('-M', '--list-modules', action='store_const', const=True, help='Show available modules and exit')
args = parser.parse_args()

# Run active modules

if args.list_modules:
    for name, module in all_modules.items():
        print("  {s.BRIGHT}{f.WHITE}{name:15s}{s.NORMAL}{f.RESET}{description}".format(f=Fore, s=Style, name=name, description=module.description))
    sys.exit(0)

active_modules = list(all_modules.values())
if args.modules:
    active_modules = [m for m in active_modules if m.modulename in args.modules]

    for name in args.modules:
        if not name in all_modules.keys():
            print("Module not found: {}".format(name))

    if not active_modules:
        print("No module selected. Exiting.")
        sys.exit(1)

while True:
    try:
        module = random.choice(active_modules)()

        if not args.no_headers:
            draw_header(module.title)
        module.run()
    except KeyboardInterrupt:
        print()
        print("Finished.")
        break
