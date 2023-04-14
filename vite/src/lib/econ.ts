import type { CurrencyFormat } from './types/CurrencyFormat'

export const currencies: {
	[key: string]: CurrencyFormat
} = {
	EUR: {
		prefix: '€ ',
		suffix: '',
		decimalPlaces: 2,
		decimalSeparator: ',',
		thousandsSeparator: '.',
	},
	DKK: {
		prefix: '',
		suffix: ' kr.',
		decimalPlaces: 2,
		decimalSeparator: ',',
		thousandsSeparator: '.',
	},
	USD: {
		prefix: '$ ',
		suffix: '',
		decimalPlaces: 2,
		decimalSeparator: '.',
		thousandsSeparator: ',',
	},
} as const
export const currency_codes = ['EUR', 'DKK', 'USD'] as const
export type CurrencyCode = typeof currency_codes[number]

export const format_currency = (amount: number, currency: CurrencyCode) => {
	const currency_format = currencies[currency]
	const formatted_amount = (amount / 10 ** currency_format.decimalPlaces)
		.toString()
		.replace('.', currency_format.decimalSeparator)
		.replace(/\B(?=(\d{3})+(?!\d))/g, currency_format.thousandsSeparator)
	return `${currency_format.prefix}${formatted_amount}${currency_format.suffix}`
}
