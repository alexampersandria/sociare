export function teleport(element: HTMLElement, query: string) {
	let teleportContainer = document.querySelector(query)
	teleportContainer.appendChild(element)

	return {
		destroy() {
			element.remove()
		},
	}
}
