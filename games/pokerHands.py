import itertools

ranks = list(range(2,11)) + ['J', 'Q', 'K', 'A']
ranks = [str(rank) for rank in ranks]

# print(ranks)

suits = ['Hearts', 'Clubs', 'Diamonds', 'Spades']
deck = [card for card in itertools.product(ranks, suits)]

# for (index, card) in enumerate(deck):
#   print(1 + index, card)

hands = [hand for hand in itertools.combinations(deck,5)]

print(f"The number of 5-card poker hands is {len(hands)}.")
