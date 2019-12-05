import argparse
from lib import compute_fuel_requirement, compute_complete_fuel_requirement

def main(filename):
	fuel_requirement = 0
	complete_fuel_requirement = 0

	with open(filename, 'r') as f:
		for line in f:
			try:
				value = int(line.strip())
			except:
				raise ValueError(f"Failed to parse {line} as int")
			fuel_requirement += compute_fuel_requirement(value)
			complete_fuel_requirement += compute_complete_fuel_requirement(value)

	print(f"Fuel required: {fuel_requirement}")
	print(f"Complete fuel required: {complete_fuel_requirement}")

if __name__ == '__main__':
	parser = argparse.ArgumentParser(description='Day 1 advent of code')
	parser.add_argument('filename', type=str, help='Input file path')
	args = parser.parse_args()

	main(args.filename)