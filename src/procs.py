from random import randint
from time import sleep
from multiprocessing import Queue, Process

N = 100

def producer(fifo):
    for x in range(N):
        sleep(0.001 * randint(0, 10))
        fifo.put(x)
        print(f"Sent {x}")
    fifo.put(None)

def consumer(fifo):
    while True:
        x = fifo.get()
        if x is None:
            break
        sleep(0.001 * randint(0, 10))
        print(f"Received {x}")

fifo = Queue()
producer_proc = Process(target=producer, args=(fifo,))
consumer_proc = Process(target=consumer, args=(fifo,))

consumer_proc.start()
producer_proc.start()

producer_proc.join()
consumer_proc.join()
