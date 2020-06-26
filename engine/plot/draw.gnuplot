file = 'simulation_100_000.csv'
set datafile separator ','
set key autotitle columnhead
plot file using 1:2 smooth csplines, \
file using 1:3 smooth csplines, \
file using 1:4 smooth csplines, \
file using 1:5 smooth csplines, \
file using 1:6 smooth csplines
