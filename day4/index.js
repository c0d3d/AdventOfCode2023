const fs = require('fs');

function parseCard(line) {
  const [cardTxt, restTxt] = line.split(":");
  const [, cardNumStr] = cardTxt.split(" ");
  const cardNum = parseInt(cardNumStr);

  const [winners, mineNums] = restTxt.trim().split("|");
  const winning = winners.trim().split(/\s+/).map((x) => parseInt(x))
        .reduce((obj, nxt) => obj[nxt] = 1 && obj, {});
  const mine = mineNums.trim().split(/\s+/).map((x) => parseInt(x));

  return { cardNum, winning, mine, copies: 1 };
}

const input = fs.readFileSync(__dirname + "/input.txt", "utf-8");
const cards = [];

let totalWinP1 = 0;
let totalWinP2 = 0;

for (const line of input.split("\n")) {
  if (line.length === 0) continue;
  const card = parseCard(line);
  cards.push(card);
}

for (let i = 0; i < cards.length; i++) {
  const card = cards[i];
  let winningNums = 0;

  // P1
  for (const myNum of card.mine) {
    if (card.winning[myNum]) {
      winningNums++;
    }
  }

  // This is too cute not to do.
  totalWinP1 += (!!winningNums) << (winningNums - 1);

  // P2
  for (let j = 1; j <= winningNums; j++) {
    cards[i + j].copies += card.copies;
  }
  totalWinP2 += card.copies;
}

console.log("Total Win (P1): " + totalWinP1);
console.log("Total Win (P2): " + totalWinP2);
