import re
from pathlib import Path

current_dir = Path(__file__).resolve().parent

reg = r'mul\([0-9]*,[0-9]*\)'

with open(current_dir / "input.txt") as f:
    text = f.read()
f.close

mult_list = re.findall(reg, text)

nb_reg = r'\d+'
sum = 0
for elem in mult_list:
    nbs = re.findall(nb_reg, elem)
    sum += int(nbs[0])*int(nbs[1])
    
print("Total Sum: ", sum)