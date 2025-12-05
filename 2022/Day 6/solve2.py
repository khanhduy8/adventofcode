def find_start_packet(inp):
    for i in range(0,len(inp)):
        packet = inp[i:i+4]
        if len(packet) == 4:
            packet = list(packet)
            dup = False
            for j in range(1,len(packet)):
                if packet[j] in packet[:j]:
                    dup = True
            if not dup:
                return i + 4
                break

def find_start_message(inp):
    for i in range(0,len(inp)):
        packet = inp[i:i+14]
        if len(packet) == 14:
            packet = list(packet)
            dup = False
            for j in range(1,len(packet)):
                if packet[j] in packet[:j]:
                    dup = True
            if not dup:
                return i + 14
                break

with open('input') as f:
    inp = f.read().strip()
    x = find_start_packet(inp)
    print(find_start_message(inp[:]))


