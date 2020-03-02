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
