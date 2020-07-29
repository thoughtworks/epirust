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
import pandas as pd
import matplotlib.pyplot as plt
from .Curve import Curve
import os


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


def toggle_visibility(figure, legend_line, plots, ax):
    for plot in plots:
        vis = not plot.get_visible()
        plot.set_visible(vis)
        if vis:
            legend_line.set_alpha(1.0)
        else:
            legend_line.set_alpha(0.2)
        ax.relim(True)
        ax.autoscale_view()
        figure.canvas.draw()


def load_collated_csv(data_frame):
    columns = list(filter(lambda c: c != 'hour' and '_std' not in c, data_frame.columns))
    return list(map(lambda c: Curve(c, data_frame[c], data_frame[f'{c}_std']), columns))


class EpiCurves:
    strategies = {
        list: calculate_mean_and_standard_dev,
        pd.DataFrame: load_collated_csv
    }

    def __init__(self, epi_curve_input):
        _class = next(filter(lambda c: isinstance(epi_curve_input, c), self.strategies), None)
        if _class is None:
            raise Exception("Input has to be list of DataFrames or DataFrames")
        self.curves = self.strategies[_class](epi_curve_input)

    def plot(self, color_mapping, title):
        fig, axes = plt.subplots(figsize=(15, 8))
        plot_lines = list(map(lambda curve: curve.plot(axes, color_mapping), self.curves))
        legend = axes.legend(bbox_to_anchor=(1, 1), loc='upper left')
        lined = dict()
        for legend_line, plot_line in zip(legend.get_lines(), plot_lines):
            legend_line.set_picker(5)
            lined[legend_line] = plot_line

        fig.canvas.mpl_connect('pick_event', lambda e: toggle_visibility(fig, e.artist, lined[e.artist], axes))
        plt.title(title)
        plt.xlabel('Days')
        plt.ylabel('No. of individuals')
        plt.show()

    def to_csv(self, output_path):
        if os.path.isdir(output_path):
            output_path = f'{output_path}/collated_simulation.csv'

        data = {}
        for curve in self.curves:
            data.update(curve.to_dictionary())

        data_frame = pd.DataFrame(data)
        data_frame['hour'] = data_frame.index + 1
        data_frame.to_csv(output_path, index=None)

    def compare_plot(self, data_frame, color_mapping):
        fig, axes = plt.subplots(figsize=(15, 8))

        plot_lines = []

        for curve in self.curves:
            time_steps = np.arange(0, curve.curve_mean.size, 24)
            color = color_mapping[curve.name]
            curve_mean = curve.curve_mean[time_steps]
            curve_std = curve.curve_std[time_steps]
            line_plot, = axes.plot(curve_mean, label=f'{curve.name}-baseline', alpha=0.5, color=color)
            poly_line = axes.fill_between(np.arange(time_steps.size), curve_mean - curve_std, curve_mean + curve_std, alpha=0.3, color=color)
            plot_lines.append([line_plot, poly_line])
            line_plot, = axes.plot(np.array(data_frame[curve.name])[time_steps], label=curve.name, color=color)
            plot_lines.append([line_plot])

        lock_down_start = data_frame[data_frame.infected >= 100].hour.min() / 24
        plt.axvspan(lock_down_start, lock_down_start + 21, alpha=0.3)
        plt.text(lock_down_start + 8, data_frame['susceptible'].max()/3, 'Lockdown Period', rotation=90)

        legend = plt.legend(bbox_to_anchor=(1, 1), loc='upper left')

        lined = dict()
        for legend_line, plot_line in zip(legend.get_lines(), plot_lines):
            legend_line.set_picker(5)
            lined[legend_line] = plot_line
            toggle_visibility(fig, legend_line, plot_line, axes)

        fig.canvas.mpl_connect('pick_event', lambda e: toggle_visibility(fig, e.artist, lined[e.artist], axes))
        plt.xlabel('Days')
        plt.ylabel('No. of individuals')
        plt.show()

    def fit_gaussian(self):
        for curve in self.curves:
            curve.fit_gaussian_to_infected()
