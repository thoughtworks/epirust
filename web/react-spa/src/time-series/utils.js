export function transformTimeSeriesMessages(message) {
    const { hour, susceptible, infected, quarantined, recovered, deceased } = message;
    const perHourStats = [hour, susceptible, infected, quarantined, recovered, deceased];
    return perHourStats;
}

export function transformTimeSeriesDeviationMessages(message) {
    const { hour, susceptible, infected, quarantined, recovered, deceased,
        infected_mean, susceptible_mean, quarantined_mean, recovered_mean, deceased_mean,
        infected_std, susceptible_std, quarantined_std, recovered_std, deceased_std } = message;
    const perHourStats = [hour, susceptible, 0, infected, 0, quarantined, 0, recovered, 0, deceased, 0,
        susceptible_mean, susceptible_std, infected_mean, infected_std, quarantined_mean, quarantined_std,
        recovered_mean, recovered_std, deceased_mean, deceased_std];
    return perHourStats;
}