with open('input') as f:
    inp = f.read().strip()
    for i in range(0,len(inp)):
        packet = inp[i:i+4]
        if len(packet) == 4:
            packet = list(packet)
            dup = False
            for j in range(1,len(packet)):
                if packet[j] in packet[:j]:
                    dup = True
            if not dup:
                print(i+4)
                break
