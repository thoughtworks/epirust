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

import matplotlib.pyplot as plt
import argparse
import pandas as pd


def arg_parser():
    parser = argparse.ArgumentParser(description='plot peaks from csv')
    parser.add_argument('--data-path', help='path to data csv file', required=True)
    parser.add_argument('--time-column', help='name of the column representing time', default='hour')
    return parser.parse_args()


def plot(data_frame, time_column):
    columns = filter(lambda c: c != time_column, data_frame.columns)
    fig, ax1 = plt.subplots()
    plot_lines = []
    for column in columns:
        plot_line, = ax1.plot(data_frame[time_column], data_frame[column], label=column)
        plot_lines.append(plot_line)

    fig.legend()
    plt.grid(True)
    plt.show()


if __name__ == '__main__':
    args = arg_parser()
    data_frame = pd.read_csv(args.data_path)
    plot(data_frame, args.time_column)

