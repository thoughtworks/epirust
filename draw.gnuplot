file = 'simulation_100_000.csv'
set datafile separator ','
set key autotitle columnhead
plot file using 1:2 with lines, \
file using 1:3 with lines, \
file using 1:4 with lines, \
file using 1:5 with lines, \
file using 1:6 with lines
