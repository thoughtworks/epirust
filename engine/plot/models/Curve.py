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

    def plot(self, axes):
        time_steps = np.arange(self.curve_mean.size)
        axes.plot(time_steps, self.curve_mean, label=self.name)
        axes.fill_between(time_steps, self.curve_mean - self.curve_std, self.curve_mean + self.curve_std, alpha=0.5)

    def to_dictionary(self):
        return {
            self.name: self.curve_mean,
            f'{self.name}_std': self.curve_std
        }