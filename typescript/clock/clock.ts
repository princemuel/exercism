export class Clock {
    private static readonly HOURS_PER_DAY = 24;
    private static readonly MINUTES_PER_HOUR = 60;
    private static readonly MINUTES_PER_DAY =
        Clock.HOURS_PER_DAY * Clock.MINUTES_PER_HOUR;

    private minutes = 0;
    constructor(hours: number, minutes: number = 0) {
        this.minutes = Clock.euclid(
            hours * Clock.MINUTES_PER_HOUR + minutes,
            Clock.MINUTES_PER_DAY,
        );
    }

    public plus(minutes: number): Clock {
        return new Clock(0, this.minutes + minutes);
    }

    public minus(minutes: number): Clock {
        return new Clock(0, this.minutes - minutes);
    }

    public equals(other: Clock): boolean {
        return this.minutes === other.minutes;
    }

    public toString(): string {
        const hours = Math.floor(this.minutes / Clock.MINUTES_PER_HOUR);
        const minutes = this.minutes % Clock.MINUTES_PER_HOUR;
        return `${Clock.pad(hours)}:${Clock.pad(minutes)}`;
    }

    private static euclid(a: number, b: number): number {
        return ((a % b) + b) % b;
    }

    private static pad(value: number): string {
        return value.toString().padStart(2, "0");
    }
}
