firstCol, secondCol = [], []
with open("day01/input.txt") as f:
    for line in f:
        elems = line.split()
        firstCol.append(elems[0])
        secondCol.append(elems[1])
f.close

sum = 0
firstCol.sort()
secondCol.sort()
sizeList = len(firstCol)
for i in range(sizeList):
    sum += abs(int(firstCol[i])-int(secondCol[i]))
    
print("Total Distance: ",sum)
    
    