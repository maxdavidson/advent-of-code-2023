export function part1(input: string) {
  return input
    .trim()
    .split("\n")
    .map((line) => line.trim())
    .map((line) => {
      let chars = Array.from(line);

      let a = chars.find(Number);
      let b = chars.findLast(Number);

      return Number(`${a}${b}`);
    })
    .reduce((a, b) => a + b, 0);
}

const DIGIT_PATTERN = /(?=(\d|one|two|three|four|five|six|seven|eight|nine))/g;

const DIGIT_VALUES: Record<string, string> = {
  one: "1",
  two: "2",
  three: "3",
  four: "4",
  five: "5",
  six: "6",
  seven: "7",
  eight: "8",
  nine: "9",
};

export function part2(input: string) {
  return input
    .trim()
    .split("\n")
    .map((line) => line.trim())
    .map((line) => {
      const matches = Array.from(line.matchAll(DIGIT_PATTERN), (m) => m[1]);
      const a = matches.at(0)!;
      const b = matches.at(-1)!;

      return Number(`${DIGIT_VALUES[a] ?? a}${DIGIT_VALUES[b] ?? b}`);
    })
    .reduce((a, b) => a + b, 0);
}
