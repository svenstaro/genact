from genact.module import Module
import time, sys
import genact.term as t

chars = [
    ('-', 2, 0, 0.05),
    ('/', 2, 1, 0.02),
    ('|', 1, 1, 0.02),
    ('/', 0, 1, 0.02),
    ('\\', 0, 2, 0.02),
    ('\\', 1, 3, 0.02),
    ('|', 2, 3, 0.02),
    ('|', 3, 3, 0.02),
    ('\\', 4, 4, 0.02),
    ('/', 4, 5, 0.02),
    ('/', 3, 6, 0.02),
    ('.', 2, 7, 0.05),
    ('_', 2, 8, 0.05),
    ('-', 2, 9, 0.05),
    ('_', 2, 10, 0.05),
    ('-', 2, 11, 0.05),
    ('_', 2, 12, 0.05),
]

class HeartbeatModule(Module):
    modulename  = "heartbeat"
    description = "print a beautiful ascii-art heartbeat"
    title       = "Medical status"

    def run(self):
        print((' '*80 + '\n')*5)
        for i in range(10):
            for c, row, col, d in chars:
                x = i * 13 + col
                y = 5 - row

                print(''.join([
                    t.cursor_up(y),
                    t.cursor_forward(x),
                    c,
                    t.cursor_down(y),
                    t.cursor_left(),
                ]), end='')
                sys.stdout.flush()

                time.sleep(d)
