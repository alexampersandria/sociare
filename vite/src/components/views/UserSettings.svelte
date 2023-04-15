<script>
	import { _ } from 'svelte-i18n'
	import { createEventDispatcher } from 'svelte'
	import { user_object } from '../../lib/stores/session'

	import Button from '../Button.svelte'
	import { gravatar } from '../../lib/gravatar'

	const dispatch = createEventDispatcher()

	const close = () => {
		dispatch('close')
	}
</script>

<div class="container">
	<Button on:click={close}>
		{$_('close')}
	</Button>
	{#if $user_object}
		<div class="user-info">
			<div class="image align-center">
				<img
					class="round"
					src={gravatar($user_object.email, 256)}
					alt="Gravatar"
				/>
				<p class="muted">
					Change your avatar using <a
						target="_blank"
						href="https://en.gravatar.com/">Gravatar</a
					>.
				</p>
			</div>
		</div>
		<pre><code>{JSON.stringify($user_object, undefined, 2)}</code></pre>
	{/if}
</div>

<style>
	.user-info .image img {
		width: 256px;
		height: 256px;
	}
</style>
