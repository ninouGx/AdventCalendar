reports = []
with open("day02/input.txt") as f:
    for line in f:
        elems = line.split()
        report = [int(elem) for elem in elems ]
        reports.append(report)
f.close

validDiffs = {1,2,3}
nbSafeReport = 0

for report in reports:
    lastMulti = 1
    isReportValid = True
    for i in range(len(report)-1):        
        diff = report[i+1]-report[i]
        multiplier = -1 if (diff < 0) else 1
        if(i == 0):
            lastMulti = multiplier
        isConstant = multiplier == lastMulti
        lastMulti = multiplier
        if(((diff * multiplier) not in validDiffs) or (not isConstant)):
            isReportValid = False
            break
    if(not isReportValid):
        continue
    nbSafeReport += 1
    
print(nbSafeReport)
        