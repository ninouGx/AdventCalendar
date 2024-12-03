import re
from pathlib import Path

current_dir = Path(__file__).resolve().parent

reg = r"do\(\)|don't\(\)|mul\(\d*,\d*\)"

with open(current_dir / "input.txt") as f:
    text = f.read()
f.close

mult_list = re.findall(reg, text)

is_enable = True

nb_reg = r'\d+'
sum = 0
for elem in mult_list:
    if (elem == "do()" or elem == "don't()"):
        is_enable = True if elem == "do()" else False
        continue
    if (is_enable):
        nbs = re.findall(nb_reg, elem)
        sum += int(nbs[0])*int(nbs[1])
    
print("Total Sum: ", sum)