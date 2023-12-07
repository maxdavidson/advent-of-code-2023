export function part1(input: string) {
  const rows = input.trimEnd().split("\n");

  const rowCount = rows.length;
  const columnCount = rows[0].length;

  const numbers = [];

  for (let y = 0; y < rowCount; y += 1) {
    const row = rows[y];
    for (let x0 = 0; x0 < columnCount; x0 += 1) {
      let number = Number.parseInt(row[x0]);
      if (!Number.isNaN(number)) {
        let x1 = x0 + 1;
        let nextNumber: number;
        while (!Number.isNaN((nextNumber = Number.parseInt(row[x1])))) {
          number = 10 * number + nextNumber;
          x1 += 1;
        }
        numbers.push({ number, y, x0, x1: x1 - 1 });
        x0 = x1;
      }
    }
  }

  let numberSum = 0;

  nextNumber: for (const { number, y: y0, x0, x1 } of numbers) {
    for (let y = y0 - 1; y <= y0 + 1; y += 1) {
      for (let x = x0 - 1; x <= x1 + 1; x += 1) {
        if (y === y0 && x0 <= x && x <= x1) {
          continue;
        }
        const char = rows[y]?.[x];
        if (char !== undefined && char !== ".") {
          numberSum += number;
          continue nextNumber;
        }
      }
    }
  }

  return numberSum;
}

export function part2(input: string) {
  const rows = input.trimEnd().split("\n");

  const rowCount = rows.length;
  const columnCount = rows[0].length;

  const numbers = new Map<number, number>();

  for (let y = 0; y < rowCount; y += 1) {
    const row = rows[y];
    for (let x0 = 0; x0 < columnCount; x0 += 1) {
      let number = Number.parseInt(row[x0]);
      if (!Number.isNaN(number)) {
        let x1 = x0 + 1;
        let nextNumber: number;
        while (!Number.isNaN((nextNumber = Number.parseInt(row[x1])))) {
          number = 10 * number + nextNumber;
          x1 += 1;
        }
        for (let x = x0; x < x1; x += 1) {
          numbers.set(y * columnCount + x, number);
        }
        x0 = x1;
      }
    }
  }

  let gearRatioSum = 0;

  for (let y = 0; y < rowCount; y += 1) {
    for (let x = 0; x < columnCount; x += 1) {
      if (rows[y][x] === "*") {
        const adjacentNumbers = new Set<number>();

        for (let dy = -1; dy <= 1; dy += 1) {
          for (let dx = -1; dx <= 1; dx += 1) {
            const number = numbers.get((y + dy) * columnCount + x + dx);
            if (number !== undefined) {
              adjacentNumbers.add(number);
            }
          }
        }

        if (adjacentNumbers.size === 2) {
          let gearRatio = 1;
          for (const number of adjacentNumbers) {
            gearRatio *= number;
          }
          gearRatioSum += gearRatio;
        }
      }
    }
  }

  return gearRatioSum;
}
