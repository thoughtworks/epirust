export const BaseColors = {
    YELLOW: "#fcce44",
    LIGHT_YELLOW: "#fff0c2",
    PURPLE: "#3498db",
    BLUE: "#007bff",
    ORANGE: "#ffa500",
    RED: "#e74c3c",
    GRAY_LIGHTER: "#f1f1f1",
    PINK: "#e83e8c",
    LIGHT_PINK: "#f9d1e4",
    GRAY: "#ccc",
    BLACK: "black",
    GREEN: "#28a745",
    WHITE: "#fff",
    LIGHT_ORANGE: "#fad5b5",
    LIGHT_BLUE: "#b9dafd"
};

export const AreaColors = {
    HOUSING: BaseColors.LIGHT_YELLOW,
    WORK: BaseColors.LIGHT_PINK,
    TRANSPORT: BaseColors.LIGHT_ORANGE,
    HOSPITAL: BaseColors.LIGHT_BLUE,
    OTHER: BaseColors.GRAY
};

export const LandmarkColors = {
    HOUSES: BaseColors.GRAY,
    OFFICES: BaseColors.GRAY
};

export const AgentStateToColor = {
    's': BaseColors.BLUE,
    'e': BaseColors.ORANGE,
    'i': BaseColors.RED,
    'r': BaseColors.GREEN,
    'd': BaseColors.BLACK
};

export const AgentStateMapper = {
    's': 'susceptible',
    'e': 'exposed',
    'i': 'infected',
    'r': 'recovered',
    'd': 'deceased'
};

export const Interventions = Object.freeze({
    LOCKDOWN: 'lockdown',
    BUILD_NEW_HOSPITAL: 'build_new_hospital',
    VACCINATION: 'vaccination',
    status: {
        LOCKDOWN_START: 'locked_down',
    }
});
