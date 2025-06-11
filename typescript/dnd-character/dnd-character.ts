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

    public static generateAbilityScore(): number {
        const rolls = Array.from(
            { length: 4 },
            () => Math.floor(Math.random() * 6) + 1,
        );

        return rolls
            .sort()
            .slice(-3)
            .reduce((sum, roll) => sum + roll, 0);
    }

    public static getModifierFor(abilityValue: number): number {
        return Math.floor((abilityValue - 10) / 2);
    }
}
