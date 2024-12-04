def getNbFromCard(card):
    winNb, ownNb = card.split("|")
    
    winNb = winNb.split(":")[1].strip().split(" ")
    winNb = list(filter(None, winNb))
    winNb = [int(nb) for nb in winNb]
    
    ownNb = ownNb.strip().split(" ")
    ownNb = list(filter(None, ownNb))
    ownNb = [int(nb) for nb in ownNb]
    
    return winNb, ownNb

def getNbOfCommonNb(winNb, ownNb):
    return len(set(winNb) & set(ownNb))

def main():
    card_list = []

    with open("input.txt", "r") as file:
        for line in file:
            card_list.append(line.strip())
    
    sum = 0
    
    for elem in card_list:
        winNb, ownNb = getNbFromCard(elem)
        sum += 2**(getNbOfCommonNb(winNb, ownNb)-1) if getNbOfCommonNb(winNb, ownNb) > 0 else 0
        
    print(sum)
    
if __name__ == "__main__":
    main()