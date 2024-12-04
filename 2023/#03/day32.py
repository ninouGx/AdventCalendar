import re

def find_whole_numbers_arround_coords(matrix, x, y):
    numbers = []
    for i in range(-1, 2):
        j = -1
        while j < 2:
            now_element = matrix[y + i][x + j]
            if now_element.isdigit():
                is_nb_going_left = matrix[y + i][x + j - 1].isdigit()
                #print(matrix[y + i][x + j - 1])
                if is_nb_going_left:
                    whole_number = re.search("[0-9]+", matrix[y + i][(x-2) + j:]).group(0)               
                else:
                    whole_number = re.search("[0-9]+", matrix[y + i][x + j:]).group(0)
                numbers.append(whole_number)
                j += len(whole_number)
            j += 1
    return numbers

def find_whole_numbers_arround_coord(matrix, x, y):
    numbers = []
    arround_coords = [(x - 1, y - 1), (x -1 , y), (x - 1, y + 1),
                      (x, y - 1), (x, y + 1),
                        (x + 1, y - 1), (x + 1, y), (x + 1, y + 1)] 
    for elem in arround_coords:
        if elem.isdigit():
            is_nb_going_left
    return numbers


def main():
    matrix = [] # 2D array by list of strings

    with open("input.txt", "r") as file:
        # Add dots all around the matrix to avoid out of range errors when checking arround a coord
        for line in file:
            line = line.strip()
            line = "." + line + "."
            matrix.append(line.strip())
        matrix.insert(0, "." * (len(matrix[0])))
        matrix.append("." * (len(matrix[0])))

    size_x = len(matrix[0])
    size_y = len(matrix)
    
    sum = 0
    total = []
    for i in range(1, size_y - 1):
        for j in range(1, size_x - 1):
            now_element = matrix[i][j]
            if now_element == '*':
                numbers = find_whole_numbers_arround_coords(matrix, j, i)
                if len(numbers) == 2:
                    total += numbers
                    sum += int(numbers[0]) * int(numbers[1])
                    if int(numbers[0]) == 251:
                        input("numbers: " + str(numbers) + " i: " + str(i) + " j: " + str(j))
    
    print(sum)
    #print(len(total))
    
if __name__ == "__main__":
    main()