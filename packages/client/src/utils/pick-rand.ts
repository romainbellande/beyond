export const pickRand = <T>(items: T[]): T =>
  items[Math.floor(Math.random() * items.length)];
