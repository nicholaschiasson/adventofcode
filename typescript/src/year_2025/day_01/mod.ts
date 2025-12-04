function modulo(num: number, mod: number): number {
  return ((num % mod) + mod) % mod;
}

export function part01(input: string): number {
  let num = 50;
  let count = 0;
  for (const line of input.split("\n")) {
    const sign = line[0] === "L" ? -1 : 1;
    num = modulo(num + (sign * Number.parseInt(line.slice(1))), 100);
    if (num === 0) {
      count++;
    }
  }
  return count;
}

export function part02(input: string): number {
  let num = 50;
  let count = 0;
  for (const line of input.split("\n")) {
    const sign = line[0] === "L" ? -1 : 1;
    const n = Number.parseInt(line.slice(1));
    const prev = sign > 0 ? num : (100 - num) % 100;
    count += Math.floor((n + prev) / 100);
    num = modulo(num + (sign * n), 100);
  }
  return count;
}
