<script>
	export let href = ''
	export let title = undefined
	export let text = ''
</script>

<a {href} {title} target="_blank">{text ? text : href}</a>
