import unittest
from lib import compute_fuel_requirement, compute_complete_fuel_requirement

class TestDayOne(unittest.TestCase):

    def test_part1(self):
        self.assertEqual(compute_fuel_requirement(12), 2)
        self.assertEqual(compute_fuel_requirement(14), 2)
        self.assertEqual(compute_fuel_requirement(1969), 654)
        self.assertEqual(compute_fuel_requirement(100756), 33583)

    def test_part2(self):
        self.assertEqual(compute_complete_fuel_requirement(12), 2)
        self.assertEqual(compute_complete_fuel_requirement(1969), 966)
        self.assertEqual(compute_complete_fuel_requirement(100756), 50346)

if __name__ == '__main__':
    unittest.main()