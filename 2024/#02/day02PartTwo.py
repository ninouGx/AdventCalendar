from typing import Union


reports = []
with open("day02/input.txt") as f:
    for line in f:
        elems = line.split()
        report = [int(elem) for elem in elems ]
        reports.append(report)
f.close

nbSafeReport = 0

def isReportSafe(report):
    validDiffs = {1,2,3}
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
    return isReportValid

def createSubReportWithoutValueAtIndex(report, i):
    subReport = []
    for index in range(len(report)):
        if(index != i):
            subReport.append(report[index])
                
    return subReport

for report in reports:
    lastMulti = 1
    isReportValid = isReportSafe(report)
    if(not isReportValid):
        for i in range(len(report)):
            subReport = createSubReportWithoutValueAtIndex(report, i)
            isReportValid = isReportSafe(subReport)
            if(isReportValid):
                break
        if(not isReportValid):
            continue
    nbSafeReport += 1
    
print(nbSafeReport)
        