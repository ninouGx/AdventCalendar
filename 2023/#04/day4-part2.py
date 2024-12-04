# a card correspond to the following:
# Card   1: 84 17 45 77 11 66 94 28 71 70 | 45 51 86 83 53 58 64 30 67 96 41 89  8 17 33 50 80 84  6  2 87 72 27 63 77
# win nb on the left, own nb on the right

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
    base_card_list = []

    with open("inputExample.txt", "r") as file:
        for line in file:
            card_list.append(line.strip())
            base_card_list.append(line.strip())
    
    i = 0
    while i < len(card_list):
        elem = card_list[i]
        winNb, ownNb = getNbFromCard(elem)
        nbOfCommonNb = getNbOfCommonNb(winNb, ownNb)
        for k in range(len(card_list)):
            print(k+1," : ", card_list[k])
        print("len(card_list): " + str(len(card_list)))
        input("nbOfCommonNb: " + str(nbOfCommonNb) + " i: " + str(i+1))
        for j in range(nbOfCommonNb):
            if i+1+j >= len(base_card_list):
                break
            card_list += [base_card_list[i+1+j]]
            base_card_list += "0"
        # sort the list of cards depending on the card number
        card_list = card_list[:i] + sorted(card_list[i:], key=lambda x: int(x.split(":")[0].strip().split(" ")[1]))
        i+=1
    
    print(len(card_list))
    
if __name__ == "__main__":
    main()