#  EpiRust
#  Copyright (c) 2020  ThoughtWorks, Inc.
#
#  This program is free software: you can redistribute it and/or modify
#  it under the terms of the GNU Affero General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  This program is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU Affero General Public License for more details.
#
#  You should have received a copy of the GNU Affero General Public License
#  along with this program.  If not, see <http://www.gnu.org/licenses/>.
#


import argparse
import pandas as pd
from models import EpiCurves


def arg_parser():
    parser = argparse.ArgumentParser(description='plot peaks from csv')
    parser.add_argument('--data-path', nargs='+', help='pattern to the path of simulation csvs', default=[])
    parser.add_argument('--output-path', help='path to saving the collated csvs', default=None)
    parser.add_argument('--collated-csv', help='path to the collated csvs', default=None)
    return parser.parse_args()


def open_data_frames(path_to_csvs):
    return list(map(lambda cf: pd.read_csv(cf), path_to_csvs))


if __name__ == '__main__':
    args = arg_parser()
    if len(args.data_path) == 0 and args.collated_csv is None:
        raise Exception('Either enter the simulation csvs or the collated csv file')
    if len(args.data_path) != 0 and args.collated_csv is not None:
        raise Exception('Either enter the simulation csvs or the collated csv file. Can not do both')

    if len(args.data_path):
        data_frames = open_data_frames(args.data_path)
        curves = EpiCurves(data_frames)
        curves.plot()

        if args.output_path is not None:
            curves.to_csv(args.output_path)

    if args.collated_csv is not None:
        EpiCurves(pd.read_csv(args.collated_csv)).plot()
