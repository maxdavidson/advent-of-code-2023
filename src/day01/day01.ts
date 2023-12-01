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

const FIRST_DIGIT = /^.*?(\d|one|two|three|four|five|six|seven|eight|nine).*$/;
const LAST_DIGIT = /^.*(\d|one|two|three|four|five|six|seven|eight|nine).*?$/;

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
      const [, a] = line.match(FIRST_DIGIT)!;
      const [, b] = line.match(LAST_DIGIT)!;

      return Number(`${DIGIT_VALUES[a] ?? a}${DIGIT_VALUES[b] ?? b}`);
    })
    .reduce((a, b) => a + b, 0);
}
