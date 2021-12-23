# https://stackoverflow.com/a/16467849/2398020
def roll(v): return (v[0],v[2],-v[1])
def turn(v): return (-v[1],v[0],v[2])
def sequence (v):
    for cycle in range(2):
        for step in range(3):  # Yield RTTT 3 times
            v = roll(v)
            yield(v)           #    Yield R
            for i in range(3): #    Yield TTT
                v = turn(v)
                yield(v)
        v = roll(turn(roll(v)))  # Do RTR

p = sequence(( 1, -3, 2))
for i in p:
    l = [(abs(x) - 1, x < 0) for x in i]
    print(str(l).lower(), end=',\n')
# q = sequence((-1,-1, 1))
# for i in sorted(zip(p,q)):
#     print(i)
