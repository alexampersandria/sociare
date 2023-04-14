<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { open_group, open_group_id } from '../../lib/stores/app'
	import { session } from '../../lib/stores/session'
	import { writable } from 'svelte/store'
	import { fly } from 'svelte/transition'
	import Message from '../Message.svelte'
	import GroupEvent from '../GroupEvent.svelte'

	import ArrowLeft from 'carbon-icons-svelte/lib/ArrowLeft.svelte'
	import { format_currency } from '../../lib/econ'

	const go_back = () => {
		open_group.set(null)
		open_group_id.set(null)
	}
	const on_keydown = (event) => {
		if (event.key === 'Enter') {
			go_back()
		}
	}

	const group_fetch_completed = writable(false)
	const get_group = () => {
		if (open_group_id) {
			group_fetch_completed.set(false)
			fetch(`${import.meta.env.VITE_API_URL}/api/v1/groups/${$open_group_id}`, {
				headers: {
					Authorization: `Bearer ${$session}`,
				},
			})
				.then((res) => res.json())
				.then((data) => {
					if (data.error) {
						alert(data.error)
					} else {
						data.sort((a, b) => {
							return a.events[0].event.created_at < b.events[0].event.created_at
								? 1
								: -1
						})
						open_group.set(data[0])
					}
					group_fetch_completed.set(true)
				})
		}
	}

	let back_button

	open_group_id.subscribe((id) => {
		if (id) {
			get_group()
		}
	})

	let container

	let scrolled = false
	const on_scroll = () => {
		scrolled = container.scrollTop > 0
	}
</script>

{#if $open_group}
	<div
		class="group theme-{$open_group.group.theme}"
		transition:fly={{ x: '100%', opacity: 1 }}
		class:scrolled
		bind:this={container}
		on:scroll={on_scroll}
	>
		<div class="head">
			<div class="back">
				<a
					class="none"
					href="javascript:void(0)"
					bind:this={back_button}
					on:click={go_back}
					on:keydown={on_keydown}><ArrowLeft size={24} /></a
				>
			</div>
			<div class="name">{$open_group.group.name}</div>
			<div class="total">
				{format_currency($open_group.group.total, $open_group.group.currency)}
			</div>
		</div>
		<div class="events">
			{#each $open_group.events.reverse() as event}
				{#if event.message || event.receipt || event.transaction}
					<Message {event} />
				{:else}
					<GroupEvent {event} />
				{/if}
			{/each}
		</div>
	</div>
{/if}

<style>
	.group {
		position: absolute;
		inset: 0;
		background-color: var(--gray-200);
		height: 100vh;
		width: 100vw;
		overflow-y: scroll;
	}

	.head {
		position: sticky;
		top: 0;
		background-color: var(--gray-200);
		transition: box-shadow 0.25s ease-in-out;
		padding: 1rem;
	}

	.scrolled .head {
		box-shadow: 0 0 3rem 3rem var(--gray-200);
	}

	.head .name,
	.head .total {
		text-align: center;
	}

	.head .name {
		font-size: 1.5rem;
		color: var(--gray-700);
	}

	.head .total {
		color: var(--gray-400);
	}

	.head .back {
		position: absolute;
		top: 50%;
		left: 1rem;
		translate: 0 -50%;
	}

	.events {
		padding: 1rem;
	}
</style>
