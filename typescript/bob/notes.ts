const answers = [
    "Whatever.",
    "Sure.",
    "Whoa, chill out!",
    "Calm down, I know what I'm doing!",
];

export const hey = (message: string): string => {
    const speech = message.trim();
    if (speech === "") return "Fine. Be that way!";

    const isQuestion = speech.endsWith("?") ? 1 : 0;
    const isYelling =
        /[A-Z]+/.test(speech) && speech == speech.toUpperCase() ? 2 : 0;

    return answers[isQuestion + isYelling];
};
