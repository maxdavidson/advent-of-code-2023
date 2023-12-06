const DIGITS = /\d+/g;

export function part1(input: string) {
  const [timeRow, distanceRow] = input.trimEnd().split("\n");

  const parseRow = (row: string) =>
    Array.from(row.matchAll(DIGITS), (m: RegExpMatchArray) => Number(m[0]));

  const times = parseRow(timeRow);
  const distances = parseRow(distanceRow);

  const records = Array.from(
    { length: Math.min(times.length, distances.length) },
    (_, i) => ({ time: times[i], distance: distances[i] }),
  );

  let product = 1;

  for (const record of records) {
    let count = 0;

    for (let time = 0; time <= record.time; time += 1) {
      const distance = time * (record.time - time);
      if (record.distance < distance) {
        count += 1;
      }
    }

    product *= count;
  }

  return product;
}

export function part2a(input: string) {
  const [timeRow, distanceRow] = input.trimEnd().split("\n");

  const parseRow = (row: string) =>
    Number(Array.from(row.matchAll(DIGITS), (m) => m[0]).join(""));

  const recordTime = parseRow(timeRow);
  const recordDistance = parseRow(distanceRow);

  let count = 0;

  for (let time = 0; time <= recordTime; time += 1) {
    const distance = time * (recordTime - time);
    if (recordDistance < distance) {
      count += 1;
    }
  }

  return count;
}

export function part2b(input: string) {
  const [timeRow, distanceRow] = input.trimEnd().split("\n");

  const parseRow = (row: string) =>
    Number(Array.from(row.matchAll(DIGITS), (m) => m[0]).join(""));

  const recordTime = parseRow(timeRow);
  const recordDistance = parseRow(distanceRow);

  for (let time = 0; time <= recordTime; time += 1) {
    const distance = time * (recordTime - time);
    if (recordDistance < distance) {
      return recordTime - time * 2 + 1;
    }
  }

  return 0;
}

export function part2c(input: string) {
  const [timeRow, distanceRow] = input.trimEnd().split("\n");

  const parseRow = (row: string) =>
    Number(Array.from(row.matchAll(DIGITS), (m) => m[0]).join(""));

  const recordTime = parseRow(timeRow);
  const recordDistance = parseRow(distanceRow);

  let minTime = 0;
  let maxTime = recordTime;

  while (minTime <= maxTime) {
    const midTime = Math.floor((minTime + maxTime) / 2);
    const distance = midTime * (recordTime - midTime);
    if (recordDistance < distance) {
      maxTime = midTime - 1;
    } else {
      minTime = midTime + 1;
    }
  }

  return recordTime - minTime * 2 + 1;
}
