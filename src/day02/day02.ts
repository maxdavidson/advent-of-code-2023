type Color = "red" | "green" | "blue";

const GAME_PATTERN = /^Game (?<index>\d+): (?<subsets>.*)$/;

export function part1(input: string) {
  let indexSum = 0;

  for (const line of input.trim().split("\n")) {
    const { index, subsets } = line.match(GAME_PATTERN)!.groups!;

    const counts: Record<Color, number> = { red: 0, green: 0, blue: 0 };

    for (const subset of subsets.split("; ")) {
      for (const group of subset.split(", ")) {
        const [count, rawColor] = group.split(" ");
        const color = rawColor as Color;
        counts[color] = Math.max(Number(count), counts[color]);
      }
    }

    if (counts.red <= 12 && counts.green <= 13 && counts.blue <= 14) {
      indexSum += Number(index);
    }
  }

  return indexSum;
}

export function part2(input: string) {
  let powerSum = 0;

  for (const line of input.trim().split("\n")) {
    const { subsets } = line.match(GAME_PATTERN)!.groups!;

    const counts: Record<Color, number> = { red: 0, green: 0, blue: 0 };

    for (const subset of subsets.split("; ")) {
      for (const group of subset.split(", ")) {
        const [count, rawColor] = group.split(" ");
        const color = rawColor as Color;
        counts[color] = Math.max(Number(count), counts[color]);
      }
    }

    powerSum += counts.red * counts.green * counts.blue;
  }

  return powerSum;
}
