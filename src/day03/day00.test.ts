import * as assert from "node:assert/strict";
import * as fs from "node:fs/promises";
import { describe, it } from "node:test";

import { part1, part2 } from "./day03.js";

const [INPUT, TEST_INPUT] = await Promise.all([
  fs.readFile(new URL("input", import.meta.url), "utf-8"),
  fs.readFile(new URL("test_input", import.meta.url), "utf-8"),
]);

describe("day03", () => {
  it("part1 works", () => {
    assert.equal(part1(TEST_INPUT), 4361);
    assert.equal(part1(INPUT), 546_563);
  });

  it("part2 works", () => {
    assert.equal(part2(TEST_INPUT), 467_835);
    assert.equal(part2(INPUT), 91_031_374);
  });
});
