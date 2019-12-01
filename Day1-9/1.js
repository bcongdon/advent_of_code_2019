const fs = require("fs");

function fuelReq(m) {
  return Math.max(0, Math.floor(m / 3) - 2);
}

function part1(modules) {
  return modules.map(fuelReq).reduce((a, b) => a + b, 0);
}

function part2(modules) {
  let p2Fuel = m => {
    let f = fuelReq(m);
    if (f > 0) {
      f += p2Fuel(f);
    }
    return f;
  };

  return modules.map(p2Fuel).reduce((a, b) => a + b, 0);
}

function main() {
  let data = fs.readFileSync("1.txt");
  let modules = data
    .toString()
    .split("\n")
    .map(line => new Number(line));
  console.log("Part 1: " + part1(modules));
  console.log("Part 2: " + part2(modules));
}

main();
