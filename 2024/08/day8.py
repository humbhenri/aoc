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

def antinodes(antenna1, antenna2):
  x1, y1 = antenna1
  x2, y2 = antenna2
  dx = x2 - x1
  dy = y2 - y1
  if dx == 0 and dy == 0:
    return []
  return [
    (x1 - dx, y1 - dy),
    (x2 + dx, y2 + dy),
  ]

def debug(antennas, nodes, maxrow, maxcol):
  output = ['.' * (maxcol+1) for _ in range(maxrow+1)]
  for antenna, places in antennas.items():
    for x, y in places:
      output[x] = output[x][:y] + antenna + output[x][y + 1:]
  for x, y in nodes:
    output[x] = output[x][:y] + '#' + output[x][y + 1:]
  print('\n'.join(output))


def all_antinodes(antennas, maxrow, maxcol):
  nodes = []
  antennas_places = list(chain.from_iterable([places for _, places in antennas.items()]))
  for antenna, places in antennas.items():
    for i in range(len(places)):
      for j in range(i + 1, len(places)):
        place1, place2 = places[i], places[j]
        for x, y in antinodes(place1, place2):
          if 0 <= x <= maxrow and 0 <= y <= maxcol and (x, y) not in antennas_places:
            nodes.append((x, y, place1, place2))

  #       nodes += antinodes(place1, place2)
  # nodes = [(x, y) for (x, y) in nodes
  #           if 0 <= x <= maxrow and 0 <= y <= maxcol
  #           and (x, y) not in antennas_places]
  return nodes


if __name__ == "__main__":
  with open('08.input', 'r') as f:
    content = f.read()
    antennas, maxrow, maxcol = parse(content)
    nodes = all_antinodes(antennas, maxrow, maxcol)
    # print(antennas)
    # print(antinodes((3, 4), (5, 5)))
    print(len(set([(x, y) for (x, y, *rest) in nodes])))

  # a=[1,2,3]
  # print(*product(a, a[1:]))
