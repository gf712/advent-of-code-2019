import operator
import math

class Parser:
	class Node:
		def __init__(self, op_code, first=None, second=None, third=None):
			self.op_code = op_code
			self.first, self.second, self.third = (first, second, third)
			self.length = 1 if self.op_code == 99 else 4

		def evaluate(self):
			if self.op_code == 1:
				op = operator.add
			elif self.op_code == 2:
				op = operator.mul
			else:
				return None

			return op(self.first, self.second), self.third

		def to_string(self):
			if self.op_code == 99:
				return "99"
			return f"{self.op_code},{self.first},{self.second},{self.third}"

	def __init__(self):
		pass

	def parse_string(self, string):
		string_operations = [int(x) for x in string.split(",")]
		i = 0
		new_state = []
		while (i < len(string_operations)):
			op_code = string_operations[i]
			if op_code not in [1,2]:
				break
				# val = string_operations[i:i+1]
			else:
				val = string_operations[i:i+4]
				val[1] = string_operations[val[1]]
				val[2] = string_operations[val[2]]

			node = self.Node(*val)
			result = node.evaluate()
			
			if result is not None:
				v, idx = result
				string_operations[idx] = v
			
			i += 4

		return ','.join([str(x) for x in string_operations])