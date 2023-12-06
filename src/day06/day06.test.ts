import * as assert from "node:assert/strict";
import * as fs from "node:fs/promises";
import { describe, it } from "node:test";

import { part1, part2a, part2b, part2c } from "./day06.js";

const [INPUT, TEST_INPUT] = await Promise.all([
  fs.readFile(new URL("input", import.meta.url), "utf-8"),
  fs.readFile(new URL("test_input", import.meta.url), "utf-8"),
]);

describe("day06", () => {
  it("part1 works", () => {
    assert.equal(part1(TEST_INPUT), 288);
    assert.equal(part1(INPUT), 3_317_888);
  });

  it("part2a works", () => {
    assert.equal(part2a(TEST_INPUT), 71_503);
    assert.equal(part2a(INPUT), 24_655_068);
  });

  it("part2b works", () => {
    assert.equal(part2b(TEST_INPUT), 71_503);
    assert.equal(part2b(INPUT), 24_655_068);
  });

  it("part2c works", () => {
    assert.equal(part2c(TEST_INPUT), 71_503);
    assert.equal(part2c(INPUT), 24_655_068);
  });
});
