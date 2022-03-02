import asyncio
import time
import pathlib
import json
from typing import Tuple, List
import pandas as pd

from motivus.client import Client

config = {"run_type": "wasm"}

def parse_light_curve(path: pathlib.Path) -> Tuple[List, List, List]:
    """
    Modify this for your particular use case
    """
    df = pd.read_csv(path, header=None, delim_whitespace=True)
    mjd, mag, err = df.values.T
    return list(mjd), list(mag), list(err)

def light_curve_to_json(path: pathlib.Path) -> str:
    mjd, mag, err = parse_light_curve(path)
    light_curve_dict = {'mjd': mjd, 'mag': mag, 'err': err}
    return json.dumps(light_curve_dict)

async def periodogram_batch():

    #motivus = await Client.connect()
    overwrite = False
    task_ids = []
    for lc_path in pathlib.Path('data').glob("*.dat"):
        result_path = pathlib.Path('results') / lc_path.name
        if overwrite or not result_path.exists():
            metadata = {'light_curve': light_curve_to_json(lc_path), 'periodogram': result_path}
            metadata.update(config)
            #task_id = motivus.call_async(metadata)
            #task_ids.append(task_id)
    if len(task_ids) > 0:
        #await motivus.barrier(tasks_ids)
        pass
    else:
        print("Tasks are done, no more tasks will be queued unless overwrite=True")
    print("Finised")
    return 1

asyncio.run(periodogram_batch())
