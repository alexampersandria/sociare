const currencies = ['EUR', 'DKK', 'USD'] as const
type currency = typeof currencies[number]

export { currencies }
export type { currency }
