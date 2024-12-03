from re import sub
# Find the sum of all numbers in the input file by removing anything that insn't a number

file_path = "/home/ninou/Documents/Dev/AdventCalendar/2023/#1/input.txt"
dictNbStr = {
    "one": "o1e",
    "two": "t2o",
    "three": "t3e",
    "four": "f4r",
    "five": "f5e",
    "six": "s6x",
    "seven": "s7n",
    "eight": "e8t",
    "nine": "n9e"
}
sum = 0
with open(file_path, "r") as file:
    for line in file:
        strWithNbStr = line
        for key, value in dictNbStr.items():
            strWithNbStr = strWithNbStr.replace(key, value)
        print("line", line, "strWithNbStr", strWithNbStr, end=" ")
        strNb = sub("[^0-9]", "", strWithNbStr)
        print("strNb", strNb, end=" ")
        trueNb = int(strNb[0] + strNb[-1])
        print("trueNb", trueNb, "\n")
        sum += trueNb
print(sum)