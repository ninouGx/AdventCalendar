# Open the file in read mode
with open('input.txt', 'r') as f:
  # Read the file's contents
  input_data = f.read()


# Parse input data and convert to a list of integers
calories = []
for line in input_data.strip().split('\n'):
  if line:
    calories.append(int(line))

# Tally the total Calories for each Elf
elf_calories = {}
current_elf = 0
current_total = 0
for c in calories:
  if c == 0:
    # Start tallying for the next Elf
    current_elf += 1
    current_total = 0
  else:
    # Add the current item's Calories to the current Elf's total
    current_total += c
    elf_calories[current_elf] = current_total

# Find the Elf carrying the most Calories
max_calories = 0
max_elf = 0
for elf, total in elf_calories.items():
  if total > max_calories:
    max_calories = total
    max_elf = elf

# Print the answer
print(f'The Elf carrying the most Calories is Elf #{max_elf} with {max_calories} Calories.')
