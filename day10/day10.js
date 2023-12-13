const _ = require('lodash');
const fs = require('fs');

const input = fs.readFileSync(__dirname + "/input.txt", "utf-8");

const lines = input.split("\n");

const NS_PIPE = "|";
const EW_PIPE = "-";
const NE_PIPE = "L";
const NW_PIPE = "J";
const SW_PIPE = "7";
const SE_PIPE = "F";
const NO_PIPE = ".";
const START = "S";

function absDiff(x, y) {
  return Math.abs(x - y);
}

function doSearch(start) {
  // Contains {node, dist}
  const workQueue = [];
  // Map node.key -> dist
  const bestPaths = {
    [start.key()]: 0,
  };

  let cur = {node: start, dist: 0};
  do {
    for (const nxt of cur.node.connected) {
      if (bestPaths[nxt.key()] == null || bestPaths[nxt.key()] > cur.dist + 1) {
        bestPaths[nxt.key()] = cur.dist + 1;
        workQueue.push({ node: nxt, dist: cur.dist + 1 });
      }
    }
    cur = workQueue.shift(); // Slow ... w/e
  } while (workQueue.length > 0);

  return bestPaths;
}

class Node {
  constructor(char, x, y) {
    this.char = char;
    this.x = x;
    this.y = y;
    this.root = this;
    this.connected = [];
  }

  isStart() {
    return this.char === START;
  }

  isPipeOrNorthward() {
    return this.char === NS_PIPE || this.char === NW_PIPE || this.char === NE_PIPE || this.char === START;
  }
  isVertPipe() {
    return this.char !== EW_PIPE && this.char !== NO_PIPE && this.char !== START;
  }

  isGround() {
    return this.char === NO_PIPE;
  }

  isConnected(otherNode) {
    if (otherNode.isGround() || this.isGround()) {
      return false;
    }

    switch (this.char) {
    case NS_PIPE:
      return otherNode.x == this.x
        && (absDiff(otherNode.y, this.y) == 1);
    case EW_PIPE:
      return otherNode.y == this.y
        && (absDiff(otherNode.x, this.x) == 1);
    case NE_PIPE:
      return (otherNode.x == this.x && this.y == otherNode.y + 1)
        || (this.x == otherNode.x - 1 && this.y == otherNode.y);
      break;
    case NW_PIPE:
      return (otherNode.x == this.x && this.y == otherNode.y + 1)
        || (this.x == otherNode.x + 1 && this.y == otherNode.y);
    case SW_PIPE:
      return (otherNode.x == this.x && this.y == otherNode.y - 1)
        || (this.x == otherNode.x + 1 && this.y == otherNode.y);
    case SE_PIPE:
      return (otherNode.x == this.x && this.y == otherNode.y - 1)
        || (this.x == otherNode.x - 1 && this.y == otherNode.y);
    case START:
      return true;
    case NO_PIPE:
    default:
      return false;
    }
  }

  connect(other) {
    this.connected.push(other);
    //other.connected.push(this);
  }

  key() {
    return `(${this.x},${this.y})`;
  }
}

const nodes = [];
let start = null;

let y = 0;
for (const line of lines) {
  if (line.length === 0) {
    continue;
  }
  let x = 0;
  let curRow = [];
  for (const c of line) {
    let n = new Node(c, x, y);
    if (n.isStart()) {
      start = n;
    }
    curRow.push(n);
    x++;
  }
  nodes.push(curRow);
  y++;
}

for (let y = 0; y < nodes.length; y++) {
  for (let x = 0; x < nodes[y].length; x++) {
    const cur = nodes[y][x];
    // North
    if (y !== 0) {
      if (nodes[y - 1][x].isConnected(cur) && cur.isConnected(nodes[y - 1][x])) {
        cur.connect(nodes[y - 1][x]);
      }
    }

    // West
    if (x !== 0) {
      if (nodes[y][x - 1].isConnected(cur) && cur.isConnected(nodes[y][x - 1])) {
        cur.connect(nodes[y][x - 1]);
      }
    }

    // South
    if (y !== nodes.length - 1) {
      if (nodes[y + 1][x].isConnected(cur) && cur.isConnected(nodes[y + 1][x])) {
        cur.connect(nodes[y + 1][x]);
      }
    }

    // East
    if (x !== nodes[y].length - 1) {
      if (nodes[y][x + 1].isConnected(cur) && cur.isConnected(nodes[y][x + 1])) {
        cur.connect(nodes[y][x + 1]);
      }
    }
  }
}

const sets = {};
let total = 0;
for (let y = 0; y < nodes.length; y++) {
  for (let x = 0; x < nodes[y].length; x++) {
    total ++;
    const n = nodes[y][x];
    sets[n.key()] = n;
  }
}

const mainLoop = new Set();
const wQ = [start];
while (wQ.length > 0) {
  const nxt = wQ.shift();
  if (!mainLoop.has(nxt)) {
    mainLoop.add(nxt);
    wQ.push(...nxt.connected);
  }
}

function find(x) {
  let tgt = sets[x.key()];
  const travelled = [];
  while (tgt !== sets[tgt.key()]) {
    travelled.push(tgt);
    tgt = sets[tgt.key()];
  }

  // Optimize and update to point directly at root
  for (const t of travelled) {
    sets[t.key()] = tgt;
  }
  return tgt;
}

function union(x, y) {
  sets[y.key()] = x;
}

// Not necessary for the solution, but just for fun.
for (let y = 0; y < nodes.length; y++) {
  for (let x = 0; x < nodes[y].length; x++) {
    const cur = nodes[y][x];
    if (mainLoop.has(cur)) {
      continue;
    }
    // North
    if (y !== 0) {
      const other = nodes[y - 1][x];
      if (!mainLoop.has(other)) {
        union(find(cur), find(other));
      }
    }

    // West
    if (x !== 0) {
      const other = nodes[y][x - 1];
      if (!mainLoop.has(other)) {
        union(find(cur), find(other));
      }
    }

    // South
    if (y !== nodes.length - 1) {
      const other = nodes[y + 1][x];
      if (!mainLoop.has(other)) {
        union(find(cur), find(other));
      }
    }

    // East
    if (x !== nodes[y].length - 1) {
      const other = nodes[y][x + 1];
      if (!mainLoop.has(other)) {
        union(find(cur), find(other));
      }
    }
  }
}

const connecteds = {};
let m = 0;
for (const key of Object.keys(sets)) {
  const root = find(sets[key]);
  if (connecteds[root.key()] == null) {
    connecteds[root.key()] = 1;
  } else {
    m = Math.max(++connecteds[root.key()], m);
  }
}

for (const k of Object.keys(sets)) {
  if (connecteds[k] === 1) {
    delete connecteds[k];
  }
}

let currentlyInside = false;
let onLine = false;
let enclosedCnt = 0;
const enclosedSet = new Set();
for (let y = 0; y < nodes.length; y++) {
  for (let x = 0; x < nodes[y].length; x++) {
    const curNode = nodes[y][x];
    if (mainLoop.has(curNode)) {
      if (curNode.isPipeOrNorthward()) {
        currentlyInside = !currentlyInside;
      }
    } else if (currentlyInside) {
      enclosedSet.add(nodes[y][x].key());
      enclosedCnt++;
    }
  }
}



function dumpNodes(d, showDist) {
  let clrIdx = 0;
  const colors = ["\033[31m", "\033[32m", "\033[33m", "\033[34m", "\033[35m", "\033[36m",
                  "\033[91m", "\033[92m","\033[93m","\033[94m","\033[95m","\033[96m"];
  const colorMap = {};
  for (let y = 0; y < nodes.length; y++) {
    for (let x = 0; x < nodes.length; x++) {
      let colorEnd = '\033[49m\033[39m';
      let color = '';

      const root = find(nodes[y][x]);
      if (!colorMap[root.key()]) {
        colorMap[root.key()] = colors[clrIdx++ % colors.length];
      }

      if (mainLoop.has(nodes[y][x])) {
        color = mainLoop.has(nodes[y][x]) ? '\033[33m' : '';
      } else if (start === nodes[y][x]) {
        color = '\033[91m';
      } else {
        color = colorMap[root.key()];
        if (enclosedSet.has(nodes[y][x].key())) {
          color = '\033[42m' + color;
        }
      }

      const current = nodes[y][x];
      if (showDist) {
        //process.stdout.write(current.char);
        if (current.char !== ".") {
          const dist = d[current.key()];
          if (Number.isInteger(dist)) {
            process.stdout.write(dist.toString().padStart(4));
          } else {
            process.stdout.write("+".padStart(4));
          }

        } else {
          process.stdout.write("   .");
        }
      } else {
        process.stdout.write(color + current.char + colorEnd);
      }

    }
    process.stdout.write("\n");
  }
}

const results = doSearch(start);
// console.log(results);
//dumpNodes(results);
//dumpNodes(results, true);
//console.dir(searchList.map((x) => x.dist));
let p1 = -1;
for (const k of Object.keys(results)) {
  p1 = Math.max(p1, results[k]);
}
console.log("P1: " + p1);
console.log("P2: " + enclosedCnt);
