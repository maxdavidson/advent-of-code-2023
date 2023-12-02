const GAME_PATTERN = /^Game (?<index>\d+): (?<subsets>.*)$/;

type Color = "red" | "green" | "blue";

export function part1(input: string) {
  const maxCountByColor: Readonly<Record<Color, number>> = {
    red: 12,
    green: 13,
    blue: 14,
  };

  let indexSum = 0;

  for (const line of input.trim().split("\n")) {
    const { groups } = line.match(GAME_PATTERN)!;
    const { index, subsets } = groups!;

    const countByColor: Record<Color, number> = { red: 0, green: 0, blue: 0 };

    for (const subset of subsets.split("; ")) {
      for (const group of subset.split(", ")) {
        const [count, rawColor] = group.split(" ");
        const color = rawColor as Color;
        countByColor[color] = Math.max(Number(count), countByColor[color]);
      }
    }

    if (
      countByColor.red <= maxCountByColor.red &&
      countByColor.green <= maxCountByColor.green &&
      countByColor.blue <= maxCountByColor.blue
    ) {
      indexSum += Number.parseInt(index);
    }
  }

  return indexSum;
}

export function part2(input: string) {
  let powerSum = 0;

  for (const line of input.trim().split("\n")) {
    const { groups } = line.match(GAME_PATTERN)!;
    const { subsets } = groups!;

    const countByColor: Record<Color, number> = { red: 0, green: 0, blue: 0 };

    for (const subset of subsets.split("; ")) {
      for (const group of subset.split(", ")) {
        const [count, rawColor] = group.split(" ");
        const color = rawColor as Color;
        countByColor[color] = Math.max(Number(count), countByColor[color]);
      }
    }

    powerSum += countByColor.red * countByColor.green * countByColor.blue;
  }

  return powerSum;
}
