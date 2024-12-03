from typing import *


seeds = []
dict_maps = []
seed_to_soil_map = []

def add_value_to_map_from_range(dict : Dict[int,int], source : int, destination : int, length : int):
    for i in range(length):
        dict[source + i] = destination + i

def get_value_from_dict(value, dict):
            

with open("day05/inputTest.txt") as f:
    seeds = f.readline().split()[1:]
    dict_map : Dict[int,int] = {}
    for line in f:    
        map_line = line.split()
        if(line == "\n"):
            continue
        if("map" in line):
            if(len(dict_map) > 0):
                dict_maps.append(dict_map)
            dict_map = {}
            continue
        
        destination_range_start = int(map_line[0])
        source_range_start = int(map_line[1])
        range_length = int(map_line[2])
        add_value_to_map_from_range(dict_map, source_range_start, destination_range_start, range_length)