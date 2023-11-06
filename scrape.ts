async function cardapi(endpoint: string): Promise<any> {
  const response = await fetch(`https://constellation.cards/api/${endpoint}`);
  const data = await response.json();
  return data;
}

function tomap(records: object[]): any {
  return records.reduce((a: object, v: any) => {
    a[v.id] = v;
    return a;
  }, {});
}

const decks_json = await cardapi("decks");
const stacks_json = await cardapi("stacks");
const faces_json = await cardapi("faces");
const cards_json = await cardapi("cards");

const decks = tomap(decks_json);
const stacks = tomap(stacks_json);
const faces = tomap(faces_json);
const cards = cards_json.map((card) => {
  card["deck"] = decks[card.deckId];
  card["stack"] = stacks[card.stackId];
  card["front"] = faces[card.frontId];
  card["back"] = faces[card.backId];
  delete card.deckId;
  delete card.stackId;
  delete card.frontId;
  delete card.backId;
  return card;
});

const output = { cards };

console.log(JSON.stringify(output, null, 2));

export {};
