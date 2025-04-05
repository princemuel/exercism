class Duration {
  secs: number;
  constructor(secs: number) {
    this.secs = secs;
  }
}

abstract class Planet {
  static readonly EARTH_YEAR_SECS: number = 365.25 * 24.0 * 60.0 * 60.0;
  abstract readonly ORBITAL_PERIOD: number;

  convert(duration: Duration): number {
    return (
      Math.round(
        (duration.secs / Planet.EARTH_YEAR_SECS / this.ORBITAL_PERIOD) * 100,
      ) / 100
    );
  }
}

class Mercury extends Planet {
  readonly ORBITAL_PERIOD = 0.2408467;
}

class Venus extends Planet {
  readonly ORBITAL_PERIOD = 0.61519726;
}

class Earth extends Planet {
  readonly ORBITAL_PERIOD = 1.0;
}

class Mars extends Planet {
  readonly ORBITAL_PERIOD = 1.8808158;
}

class Jupiter extends Planet {
  readonly ORBITAL_PERIOD = 11.862615;
}

class Saturn extends Planet {
  readonly ORBITAL_PERIOD = 29.447498;
}

class Uranus extends Planet {
  readonly ORBITAL_PERIOD = 84.016846;
}

class Neptune extends Planet {
  readonly ORBITAL_PERIOD = 164.79132;
}

class PlanetFactory {
  static readonly planets = new Map<string, Planet>([
    ["mercury", new Mercury()],
    ["venus", new Venus()],
    ["earth", new Earth()],
    ["mars", new Mars()],
    ["jupiter", new Jupiter()],
    ["saturn", new Saturn()],
    ["uranus", new Uranus()],
    ["neptune", new Neptune()],
  ]);

  static getPlanet(name: string): Planet {
    const planet = this.planets.get(name.toLowerCase());
    if (!planet) throw new Error(`Unknown planet: ${name}`);
    return planet;
  }
}

export function age(planetName: string, seconds: number): number {
  const planet = PlanetFactory.getPlanet(planetName);
  return planet.convert(new Duration(seconds));
}
