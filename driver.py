import asyncio
from motivus.client import Client

from light_curve_utils import light_curve_to_json

async def main():

    task_definition = {
        'wasm_path': './build/periodogram-0.0.wasm',
        'loader_path': './build/periodogram-0.0.js',
        'params': [
            light_curve_to_json("data/OGLE-LMC-RCB-01.dat"),
            0.1, 1.0, 0.1, 1.01
        ]
    }

    motivus = await Client.connect()

    task_id = motivus.call_async(task_definition)
    task = motivus.select_task(task_id)
    result = await task

    print(result)

if __name__ == "__main__":
    asyncio.run(main())

