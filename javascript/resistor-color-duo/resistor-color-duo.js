//
// This is only a SKELETON file for the 'Resistor Color Duo' exercise. It's been provided as a
// convenience to get you started writing code faster.
//

export function decodedValue([a, b, ..._]) {
    return ResistorValues.indexOf(a) * 10 + ResistorValues.indexOf(b);
}

const ResistorValues = [
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "violet",
    "grey",
    "white",
];
