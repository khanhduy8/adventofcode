direct = {'U':(0,1),'D':(0,-1),'L':(-1,0),'R':(1,0)}

H = [0,0]
T = [0,0]
track = ['0.0']

def update_tail(H, T, track, d, step):
    for i in range(step):
        H[0] += direct[d][0]
        H[1] += direct[d][1]
        x = abs(H[0]-T[0])
        y = abs(H[1]-T[1])
        r = x*x + y*y
        if (H[0] == T[0] and H[1] == T[1]) or 1 <= r <= 2:
            coor = str(T[0]) + '.' + str(T[1])
            if coor not in track:
                track.append(coor)
        elif r == 4:
            T[0] += direct[d][0]
            T[1] += direct[d][1]
            coor = str(T[0]) + '.' + str(T[1])
            if coor not in track:
                track.append(coor)
        elif y*y == 4:
            T[0] = H[0]
            T[1] += direct[d][1]
            coor = str(T[0]) + '.' + str(T[1])
            if coor not in track:
                track.append(coor)
        else:
            T[1] = H[1]
            T[0] += direct[d][0]
            coor = str(T[0]) + '.' + str(T[1])
            if coor not in track:
                track.append(coor)
    return T, track

with open('input') as f:
    for s in f:
        d, step = s.strip().split(' ')
        step = int(step)
        T, track = update_tail(H, T, track, d, step)
    print(track)
    print(len(track))

