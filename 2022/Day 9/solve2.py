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

def update_tail(H, T, track, d, vector):
    x = abs(H[0]-T[0])
    y = abs(H[1]-T[1])
    r = x*x + y*y
    if (H[0] == T[0] and H[1] == T[1]) or 1 <= r <= 2:
        vector = (0,0)
        coor = str(T[0]) + '.' + str(T[1])
        if coor not in track:
            track.append(coor)
    elif r == 4:
        T[0] += direct[d][0]
        T[1] += direct[d][1]
        vector = (direct[d][0],direct[d][1])
        coor = str(T[0]) + '.' + str(T[1])
        if coor not in track:
            track.append(coor)
    elif y*y == 4:
        old_T = [T[0],T[1]]
        T[0] = H[0] + vector[0]
        T[1] += vector[1]
        print(T,old_T)
        vector = (T[0]-old_T[0],T[1]-old_T[1])
        coor = str(T[0]) + '.' + str(T[1])
        if coor not in track:
            track.append(coor)
    else:
        old_T = [T[0],T[1]]
        T[0] += vector[0]
        T[1] = H[1] + vector[1]
        print(T, old_T)
        vector = (T[0]-old_T[0],T[1]-old_T[1])
        coor = str(T[0]) + '.' + str(T[1])
        if coor not in track:
            track.append(coor)
    print(vector)
    return T, track, vector

with open('test') as f:
    for s in f:
        d, step = s.strip().split(' ')
        step = int(step)
        for i in range(step):
            vector = (direct[d][0], direct[d][1])
            H[0] += direct[d][0]
            H[1] += direct[d][1]
            B1, other_track, vector = update_tail(H, B1, other_track, d, vector)
            B2, other_track, vector = update_tail(B1, B2, other_track, d, vector)
            B3, other_track, vector = update_tail(B2, B3, other_track, d, vector)
            B4, other_track, vector = update_tail(B3, B4, other_track, d, vector)
            B5, other_track, vector = update_tail(B4, B5, other_track, d, vector)
            B6, other_track, vector = update_tail(B5, B6, other_track, d, vector)
            B7, other_track, vector = update_tail(B6, B7, other_track, d, vector)
            B8, other_track, vector = update_tail(B7, B8, other_track, d, vector)
            T, other_track, vector = update_tail(B8, T, other_track, d, vector)
            print(d,step,H,B1,B2,B3,B4,B5,B6,B7,B8,T)
    print(track)
    print(len(track))

