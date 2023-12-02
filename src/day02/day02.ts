type Color = "red" | "green" | "blue";

const GAME_PATTERN = /^Game (\d+): (.*)$/gm;

function* games(input: string) {
  for (const [, index, subsets] of input.matchAll(GAME_PATTERN)) {
    yield { index: Number(index), subsets };
  }
}

function* rolls(subsets: string) {
  for (const subset of subsets.split("; ")) {
    for (const group of subset.split(", ")) {
      const [count, rawColor] = group.split(" ");
      yield { count: Number(count), color: rawColor as Color };
    }
  }
}

export function part1(input: string) {
  let indexSum = 0;

  for (const { index, subsets } of games(input)) {
    const counts: Record<Color, number> = { red: 0, green: 0, blue: 0 };

    for (const { color, count } of rolls(subsets)) {
      counts[color] = Math.max(count, counts[color]);
    }

    if (counts.red <= 12 && counts.green <= 13 && counts.blue <= 14) {
      indexSum += index;
    }
  }

  return indexSum;
}

export function part2(input: string) {
  let powerSum = 0;

  for (const { subsets } of games(input)) {
    const counts: Record<Color, number> = { red: 0, green: 0, blue: 0 };

    for (const { count, color } of rolls(subsets)) {
      counts[color] = Math.max(count, counts[color]);
    }

    powerSum += counts.red * counts.green * counts.blue;
  }

  return powerSum;
}
