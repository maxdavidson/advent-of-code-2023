const CARD_PATTERN = /^Card +(\d+): +(.+) \| +(.*)$/gm;
const WHITESPACE = /\s+/;

function* cards(input: string) {
  for (const [, index, winning, owned] of input.matchAll(CARD_PATTERN)) {
    yield {
      index: Number(index),
      winning: winning.split(WHITESPACE).map(Number),
      owned: owned.split(WHITESPACE).map(Number),
    };
  }
}

export function part1(input: string) {
  let cardSum = 0;

  for (const card of cards(input)) {
    let matches = 0;

    for (const number of card.owned) {
      if (card.winning.includes(number)) {
        matches += 1;
      }
    }

    if (matches !== 0) {
      cardSum += 2 ** (matches - 1);
    }
  }

  return cardSum;
}

export function part2(input: string) {
  const cardsOnHand = Array.from(cards(input));
  const cardCounts = Array.from(cardsOnHand, () => 1);

  for (const card of cardsOnHand) {
    let matches = 0;

    for (const number of card.owned) {
      if (card.winning.includes(number)) {
        matches += 1;
      }
    }

    for (let i = 0; i < matches; i += 1) {
      cardCounts[card.index + i] += cardCounts[card.index - 1];
    }
  }

  return cardCounts.reduce((a, b) => a + b, 0);
}
