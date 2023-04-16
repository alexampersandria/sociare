import md5 from 'js-md5'

export const gravatar = (email: string, size: number = 80) => {
	const hash = md5(email)
	return gravatar_md5(hash, size)
}

export const gravatar_md5 = (md5: string, size: number = 80) => {
	return `https://www.gravatar.com/avatar/${md5}?s=${size}&d=mp&default=identicon`
}
