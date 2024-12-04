from re import sub

file_path = "/home/ninou/Documents/Dev/AdventCalendar/2023/#2/input.txt"
color_loaded = {
    "red": 12,
    "green": 13,
    "blue": 14,
}
sum = 0
with open(file_path, "r") as file:
    for line in file:
        game_id_part, game_set_part = line.split(":")
        
        game_nb = int(sub("[^0-9]", "", game_id_part))
        
        game_set_part = game_set_part.strip()
        game_sets = game_set_part.split(";")
        
        isValid = True
        for game_set in game_sets:
            game_set = game_set.split(",")
            for pick in game_set:
                pick = pick.strip()
                nb, color = pick.split(" ")
                nb = int(nb)
                if nb > color_loaded[color]:
                    isValid = False
                    break
            if not isValid:
                break
        if isValid:
            sum += game_nb
            print(game_nb)
print(sum)