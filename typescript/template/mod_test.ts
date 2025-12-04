import { assertEquals } from "@std/assert";
import * as path from "@std/path";
import { part01, part02 } from "./mod.ts";

const challenge = import.meta.dirname?.split(path.SEPARATOR).slice(-2);
const inputPath = path.join("..", "rsrc", "inputs", ...challenge!, "tests");

Deno.test("part_01", async t => {
  await t.step("practice_1", async () => {
    assertEquals(part01((await Deno.readTextFile(path.join(inputPath, "practice_1.txt"))).trim()), 0);
  });
  await t.step("final", async () => {
    assertEquals(part01((await Deno.readTextFile(path.join(inputPath, "final.txt"))).trim()), 0);
  });
});


Deno.test("part_02", async t => {
  await t.step("practice_1", async () => {
    assertEquals(part02((await Deno.readTextFile(path.join(inputPath, "practice_1.txt"))).trim()), 0);
  });
  await t.step("final", async () => {
    assertEquals(part02((await Deno.readTextFile(path.join(inputPath, "final.txt"))).trim()), 0);
  });
});
