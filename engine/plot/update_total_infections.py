import pandas as pd
import os
import glob
import argparse
from functools import reduce

def arg_parser():
    parser = argparse.ArgumentParser(description='update total infections simulation output')
    parser.add_argument('--data-dir', help='data directory', required=True)
    parser.add_argument('--moving-avg-window',help='moving average window size (in days)', default=0, type=int, required=False)
    return parser.parse_args()

def open_data_frames(path_to_csvs):
    print(f"Processing {args.data_dir}")
    print(f"Path to CSVs {path_to_csvs}")
    ma_window=args.moving_avg_window
    for cf in path_to_csvs:
        df = pd.read_csv(cf)
        df['totalinfected'] = df['infected'] + df['recovered'] + df['deceased'] + df['hospitalized']
        if ma_window > 0:
            df['ma_infected'] = df['infected'].rolling(window=ma_window).mean()
            df['ma_deceased'] = df['deceased'].rolling(window=ma_window).mean()
        cfn = os.path.splitext(cf)
        cfn = cfn[0] + '_updated' + cfn[1]
        print(f'Writing to file {cfn}')
        df.to_csv(cfn, index=False)

if __name__ == '__main__':
	args = arg_parser()
	regions = list(filter(
		lambda x: ('interventions' not in x) and ('outgoing_travels' not in x), 
		glob.glob(f'{args.data_dir}/[a-zA-Z]*_*[0-9].csv')
	))
	open_data_frames(regions)

