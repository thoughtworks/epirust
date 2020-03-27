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

import numpy as np
import matplotlib.pyplot as plt
from .Curve import Curve


def make_number_of_rows_equal(data_frames):
    minimum_rows = min(map(lambda df: len(df.index), data_frames))
    return list(map(lambda df: df.head(minimum_rows), data_frames))


def calculate_mean_and_standard_dev(data_frames):
    df_equal_rows = make_number_of_rows_equal(data_frames)
    columns = list(filter(lambda c: c != 'hour', df_equal_rows[0].columns))
    curves = []
    for column in columns:
        collated_columns = np.array(list(map(lambda df: df[column], df_equal_rows)))
        column_mean = collated_columns.mean(axis=0)
        column_std = collated_columns.std(axis=0)
        curves.append(Curve(column, column_mean, column_std))

    return curves


class Curves:
    def __init__(self, data_frames):
        self.curves = calculate_mean_and_standard_dev(data_frames)

    def plot(self):
        fig, axes = plt.subplots()
        for curve in self.curves:
            curve.plot(axes)
        plt.legend()
        plt.show()

