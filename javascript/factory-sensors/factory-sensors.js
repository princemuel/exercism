// @ts-check

export class ArgumentError extends Error {}

export class OverheatingError extends Error {
    /**
     * @param {number} temperature
     */
    constructor(temperature) {
        super(`The temperature is ${temperature} ! Overheating !`);
        this.temperature = temperature;
    }
}

/**
 * Check if the humidity level is not too high.
 *
 * @param {number} humidityPercentage
 * @throws {Error}
 */
export const checkHumidityLevel = (humidityPercentage) => {
    if (humidityPercentage > 70) throw new OverheatingError(humidityPercentage);
};

/**
 * Check if the temperature is not too high.
 *
 * @param {number|null} temperature
 * @throws {ArgumentError|OverheatingError}
 */
export const reportOverheating = (temperature) => {
    if (temperature == null)
        throw new ArgumentError(
            `Invaliid value. Temperature cannot be ${temperature}`
        );
    if (temperature > 500) throw new OverheatingError(temperature);
};

/**
 *  Triggers the needed action depending on the result of the machine check.
 *
 * @param {{
 * check: function,
 * alertDeadSensor: function,
 * alertOverheating: function,
 * shutdown: function
 * }} actions
 * @throws {ArgumentError|OverheatingError|Error}
 */
export const monitorTheMachine = (actions) => {
    try {
        actions.check();
    } catch (error) {
        switch (error.constructor) {
            case ArgumentError:
                return actions.alertDeadSensor();
            case OverheatingError:
                if (error.temperature < 600) return actions.alertOverheating();
                return actions.shutdown();
            default:
                throw error;
        }
    }
};
