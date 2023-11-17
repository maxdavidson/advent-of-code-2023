import * as assert from "node:assert/strict";
import * as fs from "node:fs/promises";
import { describe, it } from "node:test";

import { part1, part2 } from "./day00.js";

const INPUT = await fs.readFile(new URL("input", import.meta.url), "utf-8");

describe("day00", () => {
  it("part1 works", () => {
    assert.equal(part1(INPUT), 0);
  });

  it("part2 works", () => {
    assert.equal(part2(INPUT), 0);
  });
});
