from random import randint
import asyncio

N = 10
 
async def shift(x):
    await asyncio.sleep(0.001 * randint(0, 10))
    print(f"Done with {x}")
    return 100 * x
 
async def launch():
    tasks =  [asyncio.create_task(shift(x)) for x in range(N)]
    print("All spawned")
    results = await asyncio.gather(*tasks)
    print(f"Results: {results}")
 
asyncio.run(launch())
