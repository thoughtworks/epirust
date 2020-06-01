import pandas as pd
import os
import glob
import argparse
from functools import reduce

def arg_parser():
    parser = argparse.ArgumentParser(description='integrate all the regions')
    parser.add_argument('--data-path', help='path to data csv file', required=True)
    return parser.parse_args()


def open_data_frames(path_to_csvs):
    dfs = list(map(lambda cf: pd.read_csv(cf), path_to_csvs))
    for df in dfs:
        df['totalinfected'] = df['infected'] + df['recovered'] + df['deceased'] + df['hospitalized']
    return list(map(lambda df: df.set_index('hour'), dfs))

def merge_regions(data_frames):
    return reduce(lambda x,y: x.add(y, fill_value=0.0), data_frames)


if __name__ == '__main__':
	args = arg_parser()
	
	regions = list(filter(
		lambda x: ('interventions' not in x) and ('outgoing_travels' not in x), 
		glob.glob(f'{args.data_path}/simulation_*[0-9].csv')
	))

	merge_regions(open_data_frames(regions)).to_csv(f'{args.data_path}/integrated_regions.csv')

