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

import pandas as pd
import plotly.graph_objects as go
import os

path = "/Users/in-akshaydewan/projects/e4r/results" #Change to wherever CSV are stored
folder = os.fsencode(path)

for file in os.listdir(folder):
    filename = os.fsdecode(file)
    if filename.endswith('.csv'):
        df = pd.read_csv(path + "/" + filename)
        df.head()
        fig = go.Figure()
        fig.update_layout(title=filename)
        fig.add_trace(go.Scatter(x=df['hour'], y=df['susceptible'], mode='lines', name='Susceptible'))
        fig.add_trace(go.Scatter(x=df['hour'], y=df['infected'], mode='lines', name='Infected'))
        fig.add_trace(go.Scatter(x=df['hour'], y=df['quarantined'], mode='lines', name='Quarantined'))
        fig.add_trace(go.Scatter(x=df['hour'], y=df['recovered'], mode='lines', name='Recovered'))
        fig.add_trace(go.Scatter(x=df['hour'], y=df['deceased'], mode='lines', name='Deceased'))
        fig.show()
