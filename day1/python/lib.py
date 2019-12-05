def compute_fuel_requirement(mass):
	if mass < 8:
		return 0
	return mass // 3 - 2


def compute_complete_fuel_requirement(mass):
	result = compute_fuel_requirement(mass)
	remainder = result
	while (remainder > 0):
		remainder = compute_fuel_requirement(remainder)
		result += remainder
	return result