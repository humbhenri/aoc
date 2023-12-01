import unittest

from aoc1 import part1, part2

class Test(unittest.TestCase):
    def test_part1(self):
        got = part1()
        want = 55208
        self.assertEqual(want, got)
    def test_part2(self):
        got = part2()
        want = 54578
        self.assertEqual(want, got)

if __name__ == '__main__':
    unittest.main()