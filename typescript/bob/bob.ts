const is_alphabetic = (char: string): boolean => /^\p{L}$/u.test(char);

export function hey(message: string): string {
    const msg = message.trim();
    if (!msg) return "Fine. Be that way!";

    const is_question = msg.endsWith("?");
    const has_letters = [...msg].some(is_alphabetic);
    const is_yelling = has_letters && msg === msg.toLocaleUpperCase();

    switch (true) {
        case is_yelling && is_question:
            return "Calm down, I know what I'm doing!";
        case is_yelling:
            return "Whoa, chill out!";
        case is_question:
            return "Sure.";
        default:
            return "Whatever.";
    }
}
