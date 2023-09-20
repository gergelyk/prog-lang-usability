from random import randint
from time import sleep
from threading import Thread
from queue import Queue

N = 100
T = 3

def shift(q_in, q_out, offset):
    while True:
        x = q_in.get()
        if x is None:
            break
        q_out.put(x + offset)
        sleep(0.001 * randint(0, 10))

q_in = Queue()
q_out = Queue()
offsets = ((i+1)*N for i in range(T))
threads = [Thread(target=shift, args=(q_in, q_out, offset))
           for offset in offsets]

[t.start() for t in threads]

for x in range(N):
    q_in.put(x)

[q_in.put(None) for _ in threads]

[t.join() for t in threads]

for x in range(N):
    print(q_out.get())
