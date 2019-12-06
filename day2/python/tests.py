import unittest
from lib import Parser

class TestDayTwo(unittest.TestCase):
	def test_part1_1(self):
		new_state = Parser().parse_string("1,0,0,0,99")
		self.assertEqual(new_state, "2,0,0,0,99")

	def test_part1_2(self):
		new_state = Parser().parse_string("2,3,0,3,99")
		self.assertEqual(new_state, "2,3,0,6,99")
	
	def test_part1_3(self):
		new_state = Parser().parse_string("2,4,4,5,99,0")
		self.assertEqual(new_state, "2,4,4,5,99,9801")
	
	def test_part1_4(self):
		new_state = Parser().parse_string("1,1,1,4,99,5,6,0,99")
		self.assertEqual(new_state, "30,1,1,4,2,5,6,0,99")

	def test_part1_5(self):
		new_state = Parser().parse_string("1,9,10,3,2,3,11,0,99,30,40,50")
		self.assertEqual(new_state, "3500,9,10,70,2,3,11,0,99,30,40,50")

if __name__ == '__main__':
    unittest.main()