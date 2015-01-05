from genact.module import Module
from genact.util import draw_header, draw_progress
import time, random, hashlib

passwords = ['password', 'welcome', 'qwerty', 'monkey', 'jesus', 'love', 'money', 'freedom', 'ninja'] #top10 2012 yahoo hack

class BruteforceModule(Module):
    modulename = "bruteforce"
    title = "Brute forcing Password SHA-256 hash"

    def run(self):
        password = random.choice(passwords)
        hashval = list((hashlib.sha256(password.encode('utf-8'))).hexdigest())
        hashstr = "".join(hashval)

        draw_header("SHA-Rainbow")
        print("SHA-HASH value: ", hashstr)
        draw_progress("Extracting Rainbow Table", "done", duration=2.5)
        print("Begin matching")

        match = list(" " * 64)

        while match != hashval:
            m = list(match)
            first = True

            for i in range(64):
                if m[i] == " ":
                    m[i] = random.choice("0123456789abcdef")

                    if m[i] == hashval[i] and first:
                        first = False
                        match[i] = m[i]

            print("\r " + "".join(m), end = "")
            time.sleep(0.1)

        print()
        print("Match found: {} is '{}'".format(hashstr, password))

