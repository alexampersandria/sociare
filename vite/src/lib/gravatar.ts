import md5 from 'js-md5'

export const gravatar = (email: string, size: number = 80) => {
	const hash = md5(email)
	return `https://www.gravatar.com/avatar/${hash}?s=${size}&d=mp&default=identicon`
}
