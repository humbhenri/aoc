from collections import defaultdict
from itertools import chain

def parse(content):
  antennas = defaultdict(list)
  maxrow, maxcol = 0, 0
  for i, line in enumerate(content.split('\n')):
    maxrow = max(maxrow, i)
    for j, c in enumerate(line):
      maxcol = max(maxcol, j)
      if c != '.':
        antennas[c].append((i, j))
  return antennas, maxrow, maxcol


def antinodes(antenna1, antenna2, maxrow, maxcol):
  x1, y1 = antenna1
  x2, y2 = antenna2
  dx = x2 - x1
  dy = y2 - y1
  nodes = []
  if dx == 0 and dy == 0:
    return nodes
  if x1 - dx >= 0 and y1 - dy >= 0:
    nodes.append((x1 - dx, y1 - dy))
  if x2 + dx <= maxrow and y2 + dy <= maxcol:
    nodes.append((x2 + dx, y2 + dy))
  return nodes


def antinodes_part2(antenna1, antenna2, maxrow, maxcol):
  x1, y1 = antenna1
  x2, y2 = antenna2
  dx = x2 - x1
  dy = y2 - y1
  nodes = []
  if dx == 0 and dy == 0:
    return nodes
  i=0
  while x1 - i*dx >= 0 and y1 - i*dy >= 0:
    nodes.append((x1 - i*dx, y1 - i*dy))
    i += 1
  j=0
  while x2 + j*dx <= maxrow and y2 + j*dy <= maxcol:
    nodes.append((x2 + j*dx, y2 + j*dy))
    j += 1
  return nodes


def debug(antennas, nodes, maxrow, maxcol):
  output = ['.' * (maxcol+1) for _ in range(maxrow+1)]
  for antenna, places in antennas.items():
    for x, y in places:
      output[x] = output[x][:y] + antenna + output[x][y + 1:]
  for x, y, *rest in nodes:
    output[x] = output[x][:y] + '#' + output[x][y + 1:]
  print('\n'.join(output))


def all_antinodes(antennas, maxrow, maxcol, part2=False):
  nodes = []
  antennas_places = list(chain.from_iterable([places for _, places in antennas.items()]))
  find_nodes_fn = antinodes_part2 if part2 else antinodes
  for antenna, places in antennas.items():
    for i in range(len(places)):
      for j in range(i + 1, len(places)):
        place1, place2 = places[i], places[j]
        for x, y in find_nodes_fn(place1, place2, maxrow, maxcol):
          if 0 <= x <= maxrow and 0 <= y <= maxcol:
            nodes.append((x, y, place1, place2))
  return nodes


def count_antinodes(antinodes):
  return len(set([(x, y) for (x, y, *rest) in antinodes]))


if __name__ == "__main__":
  with open('08.input', 'r') as f:
    content = f.read().rstrip()
    antennas, maxrow, maxcol = parse(content)
    antenna_count = sum(len(places) for _, places in antennas.items())
    print(f"antinodes count = {count_antinodes(all_antinodes(antennas, maxrow, maxcol))}")
    antinodes2 = all_antinodes(antennas, maxrow, maxcol, part2=True)
    print(f"antinodes count part2 = {count_antinodes(antinodes2)}")
