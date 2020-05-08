import { Interventions } from '../grid/constants';
export function parseAnnotations(interventions, hour) {

    const InterventionToClassNames = {
        [Interventions.LOCKDOWN]: "lockdown",
        [Interventions.BUILD_NEW_HOSPITAL]: "hospital",
        [Interventions.VACCINATION]: "vaccination"
    }

    function getLabel(interventionObj) {
        switch (interventionObj.intervention) {

            case Interventions.LOCKDOWN:
                return interventionObj.data.status === Interventions.status.LOCKDOWN_START
                    ? "Lockdown start"
                    : "Lockdown end"

            case Interventions.BUILD_NEW_HOSPITAL:
                return "Build Hospitals"

            case Interventions.VACCINATION:
                return "Vaccination"

            default:
                return "Unknown"
        }
    }

    return interventions.map(i => {
        const className = InterventionToClassNames[i.intervention];
        return { x: hour, label: getLabel(i), className }
    })
}

export function modelAnnotation({ x, label, className, series }, i) {
    const newLocal = i % 2 === 0;
    return {
        series: series ? series : 'susceptible',
        x,
        shortText: label,
        text: `${label} at ${x}`,
        tickHeight: newLocal ? 40 : 80,
        attachAtBottom: true,
        cssClass: `annotation ${className}`
    }
}
