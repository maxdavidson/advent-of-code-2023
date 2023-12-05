function parseInput(input: string) {
  const [rawSeeds, ...rawTransforms] = input.trimEnd().split("\n\n");

  const parseLine = (row: string) => row.split(" ").map(Number);

  const seeds = parseLine(rawSeeds.split(": ")[1]);

  const transforms = rawTransforms.map((rawTransform) =>
    rawTransform.split("\n").slice(1).map(parseLine),
  );

  return { seeds, transforms };
}

export function part1(input: string) {
  const { seeds, transforms } = parseInput(input);

  const locations = transforms.reduce((numbers, transformRanges) => {
    return numbers.map((number) => {
      for (const [dest, source, len] of transformRanges) {
        if (source <= number && number <= source + len) {
          return number + dest - source;
        }
      }

      return number;
    });
  }, seeds);

  return locations.reduce((a, b) => Math.min(a, b), Infinity);
}

export function part2(input: string) {
  const { seeds, transforms } = parseInput(input);

  const seedRanges = Array.from({ length: seeds.length / 2 }, (_, i) =>
    seeds.slice(i * 2, i * 2 + 2),
  );

  const locationRanges = transforms.reduce((ranges, transformRanges) => {
    ranges = ranges.slice();

    const nextRanges = [];

    let range: ReturnType<typeof ranges.pop>;

    nextRange: while ((range = ranges.pop()) !== undefined) {
      const rangeStart = range[0];
      const rangeEnd = range[0] + range[1];

      for (const transformRange of transformRanges) {
        const transformStart = transformRange[1];
        const transformEnd = transformRange[1] + transformRange[2];

        const largestStart = Math.max(rangeStart, transformStart);
        const smallestEnd = Math.min(rangeEnd, transformEnd);

        if (largestStart < smallestEnd) {
          if (rangeStart < largestStart) {
            ranges.push([rangeStart, largestStart - rangeStart]);
          }

          if (smallestEnd < rangeEnd) {
            ranges.push([smallestEnd, rangeEnd - smallestEnd]);
          }

          const shift = transformRange[0] - transformRange[1];
          nextRanges.push([largestStart + shift, smallestEnd - largestStart]);

          continue nextRange;
        }
      }

      nextRanges.push(range);
    }

    return nextRanges;
  }, seedRanges);

  return locationRanges.reduce(
    (smallestStart, [start]) => Math.min(smallestStart, start),
    Infinity,
  );
}
