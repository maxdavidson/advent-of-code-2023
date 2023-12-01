import * as assert from "node:assert/strict";
import * as fs from "node:fs/promises";
import { describe, it } from "node:test";

import { part1, part2 } from "./day01.js";

const INPUT = await fs.readFile(new URL("input", import.meta.url), "utf-8");

describe("day01", () => {
  it("part1 works", () => {
    assert.equal(
      part1(`
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
      `),
      142,
    );

    assert.equal(part1(INPUT), 54990);
  });

  it("part2 works", () => {
    assert.equal(
      part2(`
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
      `),
      281,
    );

    assert.equal(part2(INPUT), 54473);
  });
});
