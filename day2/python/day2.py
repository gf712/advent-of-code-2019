import argparse
from lib import Parser

def main(filename):

	with open(filename, 'r') as f:
		data = f.readlines()[0].strip() # only interested in one line

	op_codes = data.split(",")
	op_codes[1] = "12"
	op_codes[2] = "2"

	new_state = Parser().parse_string(",".join(op_codes))
	print(f"Answer for part 1: {new_state.split(',')[0]}")

	# part 2
	answer = None
	for i in range(99):
		for j in range(99):
			op_codes = data.split(",")
			op_codes[1] = str(i)
			op_codes[2] = str(j)
			new_state = Parser().parse_string(",".join(op_codes))
			result, noun, verb = new_state.split(",")[:3]
			if int(result) == 19690720:
				answer = str(100 * int(noun) + int(verb))
				break
		else:
			continue
		break 
	if answer is not None:
		print(f"Answer for part 2: {answer}")
	else:
		print("Failed to find an answer for part 2.")

if __name__ == '__main__':
	parser = argparse.ArgumentParser(description='Day 1 advent of code')
	parser.add_argument('filename', type=str, help='Input file path')
	args = parser.parse_args()

	main(args.filename)