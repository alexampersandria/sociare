<script lang="ts">
	import { createEventDispatcher } from 'svelte'
	import { gravatar_md5 } from '../lib/gravatar'
	import Spinner from './Spinner.svelte'

	export let name: string = 'user-search'

	let self

	let searching = false
	let results = []

	const on_input = () => {
		searching = true
		results = []
		let blur_value = self.value

		setTimeout(() => {
			if (blur_value === self.value && self.value.length > 0) {
				search()
			}
		}, 500)
	}

	const search = async () => {
		const res = await fetch(
			`${import.meta.env.VITE_API_URL}/api/v1/users/${self.value.replace(
				'@',
				''
			)}`
		)
		let data = await res.json()
		if (!data.error) {
			results = [data]
		}
		searching = false
	}

	const dispatch = createEventDispatcher()

	const on_select = (user) => {
		dispatch('select', user)
		results = []
		searching = false
		self.value = ''
	}
</script>

<div class="user-search wide">
	<input
		type="text"
		class="search-section"
		{name}
		bind:this={self}
		on:input={on_input}
	/>
	{#if searching || results.length > 0}
		<div class="results search-section">
			{#if searching}
				<div class="loading">
					<Spinner size={0.75} />
				</div>
			{:else}
				{#each results as user}
					<a
						href="javascript:void(0);"
						class="result none"
						on:click={() => {
							on_select(user)
						}}
					>
						<div class="gravatar">
							<img
								class="round"
								src={gravatar_md5(user.md5_email, 48)}
								alt={user.name}
							/>
						</div>
						<div class="info">
							<div class="name">{user.name}</div>
							<div class="username">@{user.username}</div>
						</div>
					</a>
				{/each}
			{/if}
		</div>
	{/if}
</div>

<style>
	.user-search {
		position: relative;
		margin-bottom: 0.25em;
	}

	.user-search input {
		position: relative;
		z-index: 2;
		margin: 0;
	}

	.user-search .search-section:first-child {
		border-radius: 0;
		border-top-left-radius: 0.25em;
		border-top-right-radius: 0.25em;
	}

	.user-search .search-section:last-child {
		border-bottom-left-radius: 0.25em;
		border-bottom-right-radius: 0.25em;
	}

	.results {
		position: absolute;
		width: 100%;
		background-color: var(--gray-300);
		z-index: 1;
	}

	.results .loading {
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.results .result {
		display: flex;
		padding: 0.5em 0.5em 0.25em 0.5em;
		color: var(--gray-500);
	}

	.results .result .gravatar img {
		width: 2.25rem;
		height: 2.25rem;
	}

	.results .result .info {
		margin-left: 0.5em;
	}

	.results .result .info .name {
		color: var(--gray-700);
	}

	.results .result .info .username {
		font-size: 0.8em;
	}
</style>
