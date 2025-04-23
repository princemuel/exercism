export class Gigasecond {
    datetime: number;

    constructor(value: Date) {
        this.datetime = Number(value);
    }
    public date(): Date {
        return new Date(this.datetime + 1e12);
    }
}
