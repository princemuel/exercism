export class DnDCharacter {
    public readonly strength: number;
    public readonly dexterity: number;
    public readonly constitution: number;
    public readonly intelligence: number;
    public readonly wisdom: number;
    public readonly charisma: number;
    public readonly hitpoints: number;

    constructor() {
        this.strength = DnDCharacter.generateAbilityScore();
        this.dexterity = DnDCharacter.generateAbilityScore();
        this.constitution = DnDCharacter.generateAbilityScore();
        this.intelligence = DnDCharacter.generateAbilityScore();
        this.wisdom = DnDCharacter.generateAbilityScore();
        this.charisma = DnDCharacter.generateAbilityScore();

        this.hitpoints = 10 + DnDCharacter.getModifierFor(this.constitution);
    }

    /**
     * Roll 4d6, drop the lowest die
     * @returns Ability score between 3-18
     */
    public static generateAbilityScore(): number {
        let sum = 0;
        let minRoll = 7;

        for (let i = 0; i < 4; i++) {
            const roll = Math.floor(Math.random() * 6) + 1;
            sum += roll;
            if (roll < minRoll) minRoll = roll;
        }

        return sum - minRoll;
    }
    /**
     * Calculate ability modifier using D&D 5e rules
     * @param score - Ability score (3-18 typically)
     * @returns Modifier (-4 to +4 for normal range)
     */
    public static getModifierFor(score: number): number {
        return Math.floor((score - 10) / 2);
    }
}
