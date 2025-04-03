
export function decodedValue( [ first, second ]: Color[] ) {
    return ResistorValues.indexOf( first ) * 10 + ResistorValues.indexOf( second )
}

type Color = typeof ResistorValues[ number ];

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
] as const
