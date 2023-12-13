function* parseInput(input: string) {
  for (const line of input.trimEnd().split("\n")) {
    const [springs, groupsRaw] = line.split(" ");
    const groups = groupsRaw.split(",").map(Number);
    yield { springs, groups };
  }
}

function createCountArrangements(springs: string, groups: readonly number[]) {
  const cache = new Map<number, number>();

  function countArrangementsRaw(
    springIndex: number,
    groupIndex: number,
  ): number {
    if (groupIndex === groups.length) {
      return springs.includes("#", springIndex) ? 0 : 1;
    }

    const group = groups[groupIndex];

    let count = 0;

    if (springs[springIndex] === "." || springs[springIndex] === "?") {
      count += countArrangements(springIndex + 1, groupIndex);
    }

    for (let index = 0; index < group; index += 1) {
      if (
        springs[springIndex + index] === undefined ||
        springs[springIndex + index] === "."
      ) {
        return count;
      }
    }

    if (springs[springIndex + group] === "#") {
      return count;
    }

    count += countArrangements(springIndex + group + 1, groupIndex + 1);

    return count;
  }

  function countArrangements(springIndex: number, groupIndex: number) {
    const key = 10_000 * springIndex + groupIndex;
    let value = cache.get(key);
    if (value !== undefined) {
      return value;
    }
    value = countArrangementsRaw(springIndex, groupIndex);
    cache.set(key, value);
    return value;
  }

  return countArrangements;
}

export function part1(input: string) {
  let totalCount = 0;

  for (const { springs, groups } of parseInput(input)) {
    const countArrangements = createCountArrangements(springs, groups);
    totalCount += countArrangements(0, 0);
  }

  return totalCount;
}

export function part2(input: string) {
  let totalCount = 0;

  for (const { springs: springsRaw, groups: groupsRaw } of parseInput(input)) {
    const springs = Array.from({ length: 5 }, () => springsRaw).join("?");
    const groups = Array.from({ length: 5 }, () => groupsRaw).flat();

    const countArrangements = createCountArrangements(springs, groups);

    totalCount += countArrangements(0, 0);
  }

  return totalCount;
}
