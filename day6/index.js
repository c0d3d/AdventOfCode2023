const fs = require('fs');

const input = fs.readFileSync(__dirname + "/input.txt", "utf-8");

const lines = input.split("\n");

let firstRow = lines[0].substring(6).split(/\s+/);
let secondRow = lines[1].substring(9).split(/\s+/);
const races = [];

for (let i = 1; i < firstRow.length; i++) {
  races.push({ time: parseInt(firstRow[i]), dist: parseInt(secondRow[i]) });
}

function distance(t, n) {
  return (t - n) * n;
}

function simplifiedQuad(b, c) {
  const rad = Math.sqrt(Math.pow(b, 2) + (4 * c));
  let first = Math.abs((-b + rad)/-2);
  let second = Math.abs((-b - rad)/-2);

  return [
    Math.ceil(first),
    Math.floor(second),
  ];
}

let outP1 = 1;
for (const race of races) {
  let raceWinWays = 0;
  let [start, stop] = simplifiedQuad(race.time, race.dist);
  for (let i = start; i < stop; i++) {
    let dis = distance(race.time, i);

    if (dis > race.dist) {
      raceWinWays++;
    }
  }
  outP1 *= raceWinWays;
}

const megaRace = {time: '', dist: ''};
races.forEach((r) => {
  megaRace.time += r.time;
  megaRace.dist += r.dist;
});
megaRace.time = parseInt(megaRace.time);
megaRace.dist = parseInt(megaRace.dist);
let outP2 = 0;
let [start, stop] = simplifiedQuad(megaRace.time, megaRace.dist);
for (let i = start; i < stop; i++) {
  let dis = distance(megaRace.time, i);
  if (dis > megaRace.dist) {
    outP2 ++;
  }
}

console.log("P1: " + outP1);
console.log("P2: " + outP2);
