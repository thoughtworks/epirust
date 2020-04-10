export function transformTimeSeriesMessages(message) {
    const { hour, susceptible, infected, quarantined, recovered, deceased } = message;
    const perHourStats = [hour, susceptible, infected, quarantined, recovered, deceased];
    return perHourStats;
}

//TODO: Modify this function according to the socket message
export function transformTimeSeriesDeviationMessages(message) {
    const { hour, susceptible, infected, quarantined, recovered, deceased } = message;
    const perHourStats = [hour, susceptible, infected, quarantined, recovered, deceased];
    return perHourStats;
}