const fs = require('fs');

const input = fs.readFileSync(__dirname + "/input.txt", "utf-8");

const lines = input.split("\n");
const growCols = new Set();
const growRows = new Set();
const GROWTH_FACTOR = 999999;

function p(x,y) {
  return `(${x},${y})`;
}

function lower(p1, p2, idx) {
  return p1[idx] < p2[idx] ? [p1, p2] : [p2, p1];
}

function canonicalize(p1, p2) {
  const pp1 = p(p1);
  const pp2 = p(p2);
  return pp1 < pp2 ? [p1, p2] : [p2, p1];
}

function pairKey(p1, p2) {
  [p1, p2] = canonicalize(p1, p2);
  return [`${p(...p1)}-${p(...p2)}`, [p1, p2]];
}

function manhattenDist(p1, p2) {
  return Math.abs(p2[0] - p1[0]) + Math.abs(p2[1] - p1[1]);
}

function megaManhatten(p1, p2, growByFactor) {
  const [firX, secX] = lower(p1, p2, 0);
  const [firY, secY] = lower(p1, p2, 1);
  let xGrowBy = 0;
  let yGrowBy = 0;

  for (let x = firX[0]; x < secX[0]; x++) {
    if (growCols.has(x)) {
      xGrowBy++;
    }
  }
  for(let y = firY[1]; y < secY[1]; y++) {
    if (growRows.has(y)) {
      yGrowBy++;
    }
  }

  const diffX = Math.abs(p2[0] - p1[0]);
  const diffY = Math.abs(p2[1] - p1[1]);
  xGrowBy *= growByFactor;
  yGrowBy *= growByFactor;

  return diffX + xGrowBy + diffY + yGrowBy;
}

const grid = [];
const rowGalaxyCounts = [];
const colGalaxyCounts = [];
const galaxies = [];
let x = 0;
let y = 0;
for (const line of lines) {
  if (line.length === 0) {
    continue;
  }
  rowGalaxyCounts[y] = 0;
  const row = [];
  x = 0;
  for (const cell of line) {
    if (colGalaxyCounts[x] == null) {
      colGalaxyCounts[x] = 0;
    }
    row.push(cell);
    if (cell === "#") {
      galaxies.push([x, y]);
      rowGalaxyCounts[y]++;
      colGalaxyCounts[x]++;
    }
    x++;
  }
  grid.push(row);
  y++;
}

let addedCols = 0;
let addedRows = 0;

colGalaxyCounts.forEach((v, idx) => {
  if (v === 0) {
    growCols.add(idx);
  }
});

rowGalaxyCounts.forEach((v, idx) => {
  if (v === 0) {
    growRows.add(idx);
  }
});

const pairs = {};
let totalMD = 0;
let totalMegaMD = 0;
for (const g1 of galaxies) {
  for (const g2 of galaxies) {
    if (g1 === g2) continue;
    const [k, [cG1, cG2]] = pairKey(g1, g2);
    if (!pairs[k]) {
      pairs[k] = [cG1, cG2];
      totalMD += megaManhatten(cG1, cG2, 1);
      totalMegaMD += megaManhatten(cG1, cG2, GROWTH_FACTOR);
    }
  }
}

console.log({growCols, growRows});
console.log("P1: " + totalMD);
console.log("P2: " + totalMegaMD);
