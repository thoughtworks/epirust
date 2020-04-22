# Plot the simulations

## Prerequisites
* Python3
* Matplotlib
* Pandas
* Numpy


## Plot one particular simulation

To visualise one particular simulation:
```bash
python plot.py --data-path <PATH_TO_CSV_FILE>
```

Sample output:
![Epicurves plot](https://user-images.githubusercontent.com/16804955/79959420-bf29a500-84a1-11ea-844e-11cc721b7b79.png)

# Plot the average of all simulations

To calculate the average of all the simulations and visualise it:
```bash
python collate_all_simulations.py --data-path <PATTERN_TO_CSVS>
```

Example:
```bash
python collate_all_simulations.py --data-path ./simulation*.csv
```

Or you can pass particular files as well
Example:
```bash
python collate_all_simulations.py --data-path ./simulation1.csv ./simulation2.csv
```

Sample output:
![Mean and deviation plot](https://user-images.githubusercontent.com/16804955/79537230-76d44680-809f-11ea-88b3-d868118b3c5d.png)

The shaded region represents the standard deviation around the mean of the curve at one particular time

To save the calculated mean and standard deviation to csv file:
```bash
python collate_all_simulations.py --data-path <PATTERN_TO_CSVS> --output-path <PATH_TO_OUTPUT_CSV>
```

Example:
```bash
python collate_all_simulations.py --data-path ./simulation*.csv --output-path ./mean_simulation.csv
```

Note: if `--output-path` is a directory e.g `./` then the file saved would be `./collated_simulation.csv`

### Plot already calculated csv file

To visualise already generated average and standard deviation calculated csv:
```bash
python collate_all_simulations.py --collated-csv <PATH_TO_COLLATED_CSV>
```

Example:
```bash
python collate_all_simulations.py --collated-csv ./mean_simulation.csv
```


# Plot comparison of a particular simulation to the baseline simulations  

To visualise already generated average and standard deviation calculated csv:
```bash
python collate_all_simulations.py --data-path <PATTERN_TO_CSVS> --compare-with <PATH_TO_CSV_To_COMPARE>
```

Example:
```bash
python collate_all_simulations.py --data-path ./simulation*.csv --compare-with ./simulation_to_compare.csv
```

Sample output:
![Comparison plot](https://user-images.githubusercontent.com/16804955/79548791-9d9c7800-80b3-11ea-9596-ad71633bb5e0.png)

In this plot:
* We can disable certain lines using the legend (by clicking the line we want to enable/disable)
* The lockdown is shown according to the simulation that is being compared(not baseline).
* The lockdown starts when number of infected reach above 100 and remains for 21 days