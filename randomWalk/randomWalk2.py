import random

def random_walk2(n):
    """Return coordinates after 'n' block random walk"""
    x, y = 0, 0
    for i in range(n):
        (dx, dy) = random.choice([(0, 1), (0, -1), (1, 0), (-1, 0)])
        x += dx
        y += dy
    return (x, y)

for i in range(25):
    walk = random_walk2(10)
    print(walk, "Distance from home = ", abs(walk[1]))
