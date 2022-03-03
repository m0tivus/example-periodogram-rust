import time
import pathlib
import json
from typing import Tuple, List, Union
import pandas as pd


def parse_light_curve(path: pathlib.Path) -> Tuple[List, List, List]:
    """
    Modify this for your particular use case
    """
    df = pd.read_csv(path, header=None, delim_whitespace=True)
    mjd, mag, err = df.values.T
    return list(mjd), list(mag), list(err)

def light_curve_to_json(path: Union[str, pathlib.Path]) -> str:
    if isinstance(path, str):
        path = pathlib.Path(path)
    mjd, mag, err = parse_light_curve(path)
    light_curve_dict = {'mjd': mjd, 'mag': mag, 'err': err}
    return json.dumps(light_curve_dict)


