import os, codecs

def esc(f, *args, **kwargs):
    return ('\u001b[' + f).format(*args, **kwargs)

def cursor_to(x=None, y=None):
    if x == None:
        return esc('H')
    elif y == None:
        return esc('{}G', x + 1)
    else:
        return esc('{};{}H', y + 1, x + 1)

def cursor_move(x, y):
    return esc('{}{}', abs(x), 'D' if x < 0 else 'C') + esc('{}{}', abs(y), 'A' if y < 0 else 'B')

def cursor_up(count = 1):
    return esc('{}A', count)

def cursor_down(count = 1):
    return esc('{}B', count)

def cursor_forward(count = 1):
    return esc('{}C', count)

def cursor_backward(count = 1):
    return esc('{}D', count)

def cursor_left():
    return esc('1000D')

def cursor_save_position():
    return esc('s')

def cursor_restore_position():
    return esc('u')

def cursor_get_position():
    return esc('6n')

def cursor_next_line():
    return esc('E')

def cursor_prev_line():
    return esc('F')

def cursor_hide():
    return esc('?25l')

def cursor_show():
    return esc('?25h')

def erase_lines(count):
    return cursorUp().join([cursorLeft() + eraseEndLine()] * count)

def erase_end_line():
    return esc('K')

def erase_start_line():
    return esc('1K')

def erase_line():
    return esc('2K')

def erase_down():
    return esc('J')

def erase_up():
    return esc('1J')

def erase_screen():
    return esc('2J')

def scroll_up():
    return esc('S')

def scroll_down():
    return esc('T')

def clear_screen():
    return '\u001bc'

def beep():
    return '\u0007'

def image(data, width=None, height=None, preserve_aspect_ratio=True):
    r = '\u001b]1337;File=inline=1'
    if width: r += ';width={}'.format(width)
    if height: r += ';height={}'.format(height)
    if not preserve_aspect_ratio: r += 'preserveAspectRatio=0;'
    r += ':' + codecs.encode(data, 'base64') + '\u0007'
    return r

def iterm_set_cwd(cwd=os.getcwd()):
    return '\u001b]50;CurrentDir={}\u0007'.format(cwd)
