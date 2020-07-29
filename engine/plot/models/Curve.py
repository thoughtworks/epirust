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
from scipy.optimize import curve_fit
from scipy.special import erf
import matplotlib.pyplot as plt


def pdf(x):
    return 1/np.sqrt(2*np.pi) * np.exp(-x**2/2)


def cdf(x):
    return (1 + erf(x/np.sqrt(2))) / 2


def skew(time, amplitude, mean, covariance, alpha):
    t = (time - mean) / covariance
    return amplitude * (2 / covariance * pdf(t) * cdf(alpha * t))


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
        if(self.name.startswith('ma_')):
            line_plot, = axes.plot(x, plot_mean, '--', label=self.name, color=color_mapping[self.name])
        else:
            line_plot, = axes.plot(x, plot_mean, label=self.name, color=color_mapping[self.name])

        fill_plot = axes.fill_between(x, plot_mean - plot_std, plot_mean + plot_std, alpha=0.5, color=color_mapping[self.name])
        return [line_plot, fill_plot]

    def to_dictionary(self):
        return {
            self.name: self.curve_mean,
            f'{self.name}_std': self.curve_std
        }

    def fit_gaussian_to_infected(self):
        if self.name != 'infected':
            return False

        time = np.arange(self.curve_mean.size)
        peak = np.where(self.curve_mean == self.curve_mean.max())[0][0]
        initial_amplitude = self.curve_mean.max()
        initial_mean = peak
        initial_sigma = 10
        initial_alpha = 5
        initial_guess = (
            initial_amplitude,
            initial_mean,
            initial_sigma,
            initial_alpha
        )

        params_fit, params_covariance = curve_fit(
            skew,
            time,
            self.curve_mean,
            p0=initial_guess,
            maxfev=10000
        )

        params = ('Amplitude', 'Mean(μ)', 'Standard Deviation(σ)', 'Skewness(α)')

        amplitude, mean, standard_deviation, skewness = params_fit

        for param, param_fit in zip(params, params_fit):
            print(f'{param}: {param_fit}')

        skew_distribution = skew(time, *params_fit)
        peak_value = skew_distribution.max()
        time_at_peak, = np.where(skew_distribution == peak_value)
        time_at_peak = time_at_peak[0]
        print(f'Peak value: {peak_value}')
        print(f'Time at peak: {time_at_peak}')

        plt.plot(time, self.curve_mean, label='actual')
        plt.plot(time, skew_distribution, label='fit skew distribution')
        plt.vlines(x=time_at_peak, ymin=0, ymax=peak_value, label='Mode')
        plt.vlines(x=mean, ymin=0, ymax=self.curve_mean[int(mean)], colors='green', label='Mean(μ)')
        two_sigma = 2 * standard_deviation
        plt.vlines(x=mean - two_sigma, ymin=0, ymax=self.curve_mean[int(mean - two_sigma)], colors='red',
                   label='Standard Deviation(2σ)')
        plt.vlines(x=mean + two_sigma, ymin=0, ymax=self.curve_mean[int(mean + two_sigma)], colors='red')
        plt.legend()
        plt.show()
