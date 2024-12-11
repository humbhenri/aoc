#!/usr/bin/env python3

import tkinter as tk
from collections import defaultdict
from day8 import parse, all_antinodes

antennas = defaultdict(list)
rows, cols = 20, 10
grid = []

def update_grid():
    global grid, antennas, rows, cols
    antinodes = all_antinodes(antennas, rows-1, cols-1)
    for antenna, places in antennas.items():
        for r,c in places:
            grid[r][c]['text'] = antenna
    for r,c in antinodes:
       if grid[r][c]['text'] == '.':
           grid[r][c]['text'] = '#'


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


def find_antenna(antennas, pos):
    for antenna, places in antennas.items():
        if pos in places:
            return antenna


if __name__=='__main__':
    root = tk.Tk()
    root.title('Advent of Code 2024 - Day 8')
    for r in range(rows):
        row = []
        for c in range(cols):
            char = '.'
            button = tk.Button(root, text=char, width=5, height=2,
                            command=lambda r=r, c=c: edit_character(r, c))
            button.grid(row=r, column=c, padx=5, pady=5)
            row.append(button)
        grid.append(row)

    # Start the main event loop
    root.mainloop()
