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


class Curve:
    def __init__(self, name, curve_mean, curve_std):
        self.name = name
        self.curve_mean = curve_mean
        self.curve_std = curve_std

    def plot(self, axes, color_mapping):
        time_steps = np.arange(0, self.curve_mean.size, 24)
        plot_mean = self.curve_mean[time_steps]
        plot_std = self.curve_std[time_steps]
        x = np.arange(plot_mean.size)
        line_plot, = axes.plot(x, plot_mean, label=self.name, color=color_mapping[self.name])
        fill_plot = axes.fill_between(x, plot_mean - plot_std, plot_mean + plot_std, alpha=0.5, color=color_mapping[self.name])
        return [line_plot, fill_plot]

    def to_dictionary(self):
        return {
            self.name: self.curve_mean,
            f'{self.name}_std': self.curve_std
        }