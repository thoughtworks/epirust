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
import json


def arg_parser():
    parser = argparse.ArgumentParser(description='plot peaks from CSV')
    parser.add_argument('--data-path', nargs='+', help='pattern to the path of simulation CSVs', default=[])
    parser.add_argument('--output-path', help='path to saving the collated CSVs', default=None)
    parser.add_argument('--collated-csv', help='path to the collated CSV', default=None)
    parser.add_argument('--compare-with', help='path to the CSV to be compared with', default=None)
    parser.add_argument('--color-mapping', help='path to the CSV to be compared with', default='color_mapping.json')
    parser.add_argument('--title', help='title of the plot', default='Default Title')
    parser.add_argument('--fit-gaussian', help='flag to fit gaussian to infected curve', action='store_true')
    return parser.parse_args()


def open_data_frames(path_to_csvs):
    return list(map(lambda cf: pd.read_csv(cf), path_to_csvs))


if __name__ == '__main__':
    args = arg_parser()
    if len(args.data_path) == 0 and args.collated_csv is None:
        raise Exception('Either enter the simulation CSVs or the collated CSV file')
    if len(args.data_path) != 0 and args.collated_csv is not None:
        raise Exception('Either enter the simulation CSVs or the collated CSV file. Can not do both')

    with open(args.color_mapping) as f:
        color_mapping = json.load(f)

    epi_curves = None
    if len(args.data_path):
        data_frames = open_data_frames(args.data_path)
        epi_curves = EpiCurves(data_frames)

        if args.output_path is not None:
            epi_curves.to_csv(args.output_path)

    if args.collated_csv is not None:
        epi_curves = EpiCurves(pd.read_csv(args.collated_csv))

    if args.compare_with is not None:
        epi_curves.compare_plot(pd.read_csv(args.compare_with), color_mapping)
    else:
        epi_curves.plot(color_mapping, args.title)

    if args.fit_gaussian:
        epi_curves.fit_gaussian()

