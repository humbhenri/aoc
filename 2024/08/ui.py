#!/usr/bin/env python3

import tkinter as tk
from collections import defaultdict
from day8 import parse, all_antinodes

antennas = defaultdict(list)
antinodes = []
rows, cols = 20, 10
grid = []

def center_window(window):
    window.update_idletasks()  # Ensure accurate geometry calculations
    width = window.winfo_width()
    height = window.winfo_height()
    screen_width = window.winfo_screenwidth()
    screen_height = window.winfo_screenheight()
    x = (screen_width // 2) - (width // 2)
    y = (screen_height // 2) - (height // 2)
    window.geometry(f"{width}x{height}+{x}+{y}")


def update_grid():
    global grid, antennas, rows, cols, antinodes, text_widget

    # erase current antinodes
    for x, y, *rest in antinodes:
        grid[x][y]['text'] = '.'
    antinodes = all_antinodes(antennas, rows-1, cols-1)

    for antenna, places in antennas.items():
        for r,c in places:
            grid[r][c]['text'] = antenna
    for (r,c,*rest) in antinodes:
       if grid[r][c]['text'] == '.':
           grid[r][c]['text'] = '#'

    text_widget.delete("1.0", tk.END)  # Clear all text in the widget
    text_widget.insert("1.0", '# antinodes: ' + str(len(antinodes)))


def edit_character(row, col):
    """Open a new top-level window to input a character."""
    def update_character():
        new_char = entry.get()
        if len(new_char) == 1:  # Ensure only one character is entered
            old_char = grid[row][col]['text']
            grid[row][col]['text'] = new_char
            if new_char != old_char:
                print(f"old_char = {old_char}, new_char = {new_char}, {antennas[old_char]}")
                if old_char in antennas and (row, col) in antennas[old_char]:
                    antennas[old_char].remove((row, col))
                if new_char not in ['.', '#']:
                    antennas[new_char].append((row, col))
            update_grid()

        editor.destroy()

    # Create a new window for input
    editor = tk.Toplevel(root)
    editor.title("Edit Character")
    tk.Label(editor, text="Enter a new character:").pack(pady=5)
    entry = tk.Entry(editor)
    entry.pack(pady=5)
    entry.focus_set()
    tk.Button(editor, text="OK", command=update_character).pack(pady=5)
    editor.update_idletasks()
    editor.geometry("300x150")  # Set an initial size
    center_window(editor)


def find_antenna(pos):
    for antenna, places in antennas.items():
        if pos in places:
            return antenna


def on_mouse_over(e, r, c):
    # e.widget['bg'] = 'red'
    for x, y, antenna1, antenna2 in antinodes:
        if r == x and y == c:
            i, j = antenna1
            grid[i][j]['bg'] = 'red'
            k, l = antenna2
            grid[k][l]['bg'] = 'red'


def on_mouse_leave(e, r, c):
    default_bg = tk.Button().cget('background')
    for x, y, antenna1, antenna2 in antinodes:
        if r == x and y == c:
            i, j = antenna1
            grid[i][j]['bg'] = default_bg
            k, l = antenna2
            grid[k][l]['bg'] = default_bg


if __name__=='__main__':
    root = tk.Tk()
    root.title('Advent of Code 2024 - Day 8')
    for r in range(rows):
        row = []
        for c in range(cols):
            char = '.'
            button = tk.Button(root, text=char, width=2, height=2,
                            command=lambda r=r, c=c: edit_character(r, c))
            button.grid(row=r, column=c, padx=5, pady=5)
            button.bind("<Enter>", lambda e, r=r, c=c: on_mouse_over(e, r, c))
            button.bind("<Leave>", lambda e, r=r, c=c: on_mouse_leave(e, r, c))
            row.append(button)
        grid.append(row)

    text_widget = tk.Text(root, height=1, width=40)
    text_widget.insert("1.0", "# antinodes: ")
    text_widget.grid(row=rows, column=0, columnspan=cols, padx=5, pady=10)

    center_window(root)
    root.mainloop()
