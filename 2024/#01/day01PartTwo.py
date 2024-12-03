firstCol, secondCol = [], []
with open("day01/input.txt") as f:
    for line in f:
        elems = line.split()
        firstCol.append(elems[0])
        secondCol.append(elems[1])
f.close

sum = 0
for nb in firstCol:
    sum += abs(int(nb)*secondCol.count(nb))
    
print("Total Similarity: ",sum)
    
    