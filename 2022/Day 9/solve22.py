direct = {'U':(0,1),'D':(0,-1),'L':(-1,0),'R':(1,0)}

H = [0,0]
B1 = [0,0]
B2 = [0,0]
B3 = [0,0]
B4 = [0,0]
B5 = [0,0]
B6 = [0,0]
B7 = [0,0]
B8 = [0,0]
T = [0,0]
track = ['0.0']
other_track = ['0.0']

def update_tail(H, T, track, d):
    x = abs(H[0]-T[0])
    y = abs(H[1]-T[1])
    r = x*x + y*y
    if (H[0] == T[0] and H[1] == T[1]) or 1 <= r <= 2:
        coor = str(T[0]) + '.' + str(T[1])
        if coor not in track:
            track.append(coor)
    else:
        x = H[0] - T[0]
        y = H[1] - T[1]
        if x != 0:
            x = int(x/abs(x))
        if y != 0:
            y = int(y/abs(y))
        T[0] += x
        T[1] += y
        coor = str(T[0]) + '.' + str(T[1])
        if coor not in track:
            track.append(coor)
    return T, track

with open('test') as f:
    for s in f:
        d, step = s.strip().split(' ')
        step = int(step)
        for i in range(step):
            H[0] += direct[d][0]
            H[1] += direct[d][1]
            B1, other_track = update_tail(H, B1, other_track, d)
            B2, other_track = update_tail(B1, B2, other_track, d)
            B3, other_track = update_tail(B2, B3, other_track, d)
            B4, other_track = update_tail(B3, B4, other_track, d)
            B5, other_track = update_tail(B4, B5, other_track, d)
            B6, other_track = update_tail(B5, B6, other_track, d)
            B7, other_track = update_tail(B6, B7, other_track, d)
            B8, other_track = update_tail(B7, B8, other_track, d)
            T, track = update_tail(B8, T, track, d)
    print(len(track))

