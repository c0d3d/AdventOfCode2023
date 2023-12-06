const fs = require('fs');
const assert = require('assert');

function parseSeeds(line) {
  let [, nums] = line.split(": ");
  return nums.split(" ").map((x) => parseInt(x));
}

function parseMapLine(line) {
  assert(line.endsWith("map:"));
  let [left] = line.split(" ");
  let [from, to] = left.split("-to-");
  return [from, to];
}

const maps = {};

function setMap(from, to, range) {
  if (!maps[from]) maps[from] = {};
  if (!maps[from][to]) maps[from][to] = [];
  maps[from][to].push(range);
}

function mapLookup(from, to, seed) {
  for (const { sourceStart, destStart, length } of maps[from][to]) {
    if (seed >= sourceStart && seed < (sourceStart + length)) {
      return destStart + (seed - sourceStart);
    }
  }

  return seed;
}

function soilToLocation(seed) {
  seed = mapLookup("seed", "soil", seed);
  seed = mapLookup("soil", "fertilizer", seed);
  seed = mapLookup("fertilizer", "water", seed);
  seed = mapLookup("water", "light", seed);
  seed = mapLookup("light", "temperature", seed);
  seed = mapLookup("temperature", "humidity", seed);
  return mapLookup("humidity", "location", seed);
}

function soilRangeToLocation(seedStart, seedLen) {
  // 1. Find earliest range in current map that fits, will always match range start, not seed start
  // 2. Try recurse
  let curMap = maps["seed"]["soil"];

}

const input = fs.readFileSync(__dirname + "/input.txt", "utf-8");

let lines = input.split("\n");
const seeds = parseSeeds(lines[0]);

let i = 2;
while(i < lines.length) {
  let mapTitle = lines[i];
  let [fromName, toName] = parseMapLine(lines[i++]);
  while (lines[i].length > 0) {
    let [destStart, sourceStart, length] = lines[i++].split(" ");
    setMap(fromName, toName,
           { destStart: parseInt(destStart), sourceStart: parseInt(sourceStart), length: parseInt(length) });
  }
  // Skip empty line
  i++;
}

// P1
let minLoc = 1000000000;
for (const nxtSeed of seeds) {
  const outLoc = soilToLocation(nxtSeed);
  minLoc = Math.min(outLoc, minLoc);
}

// P2
let idx = 0;
let minLoc2 = 100000090000;
while (idx < seeds.length) {
  let startSeedRange = seeds[idx++];
  let seedRangeLen = seeds[idx++];
  for (let nxtSeed = startSeedRange; nxtSeed < startSeedRange + seedRangeLen; nxtSeed++) {
    const outLoc = soilToLocation(nxtSeed);
    minLoc2 = Math.min(outLoc, minLoc2);
  }
}

console.log(minLoc);
console.log(minLoc2);
