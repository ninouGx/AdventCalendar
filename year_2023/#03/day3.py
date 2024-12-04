import re

def is_arround_coords_special_char(matrix, x, y):
    for i in range(-1, 2):
        for j in range(-1, 2):
            now_element = matrix[y + i][x + j]
            # Return true if there is something different than a dot or a number (a special char) 
            #input("now_element: " + now_element + " i: " + str(i) + " j: " + str(j))
            if now_element != "." and not now_element.isdigit():
                return True                
    return False


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
    
    for i in range(1, size_y - 1):
        j = 0
        while j < size_x - 1:
            now_element = matrix[i][j]
            if re.match("[0-9]", now_element):
                whole_number = re.search("[0-9]+", matrix[i][j:]).group(0)
                for k in range(len(whole_number)):
                    if is_arround_coords_special_char(matrix, j + k, i):
                        #input("whole_number: " + whole_number + " i: " + str(i) + " j: " + str(j))
                        sum += int(whole_number)
                        break
                j += len(whole_number)
            j += 1              
    
    print(sum)
    
if __name__ == "__main__":
    main()