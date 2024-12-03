from re import sub

file_path = "/home/ninou/Documents/Dev/AdventCalendar/2023/#2/input.txt"

sum = 0
with open(file_path, "r") as file:
    for line in file:
        game_id_part, game_set_part = line.split(":")
        
        game_nb = int(sub("[^0-9]", "", game_id_part))
        
        game_set_part = game_set_part.strip()
        game_sets = game_set_part.split(";")
        
        min_color_loaded = {
            "red": 1,
            "green": 1,
            "blue": 1,
        }
        
        isValid = True
        for game_set in game_sets:
            game_set = game_set.split(",")
            for pick in game_set:
                pick = pick.strip()
                nb, color = pick.split(" ")
                nb = int(nb)
                if nb > min_color_loaded[color]:
                    min_color_loaded[color] = nb
        power = 1
        for color in min_color_loaded:
            power *= min_color_loaded[color]
        sum += power
        print(power, " : ", min_color_loaded)
print(sum)