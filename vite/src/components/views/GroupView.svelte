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
	import { createForm } from 'felte'

	const go_back = () => {
		open_group.set(null)
		open_group_id.set(null)
		scrolled = false
		bottomed = false
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

	const send_message = async (content) => {
		if (open_group_id && content) {
			const res = await fetch(
				`${import.meta.env.VITE_API_URL}/api/v1/message/create`,
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Authorization: `Bearer ${$session}`,
					},
					body: JSON.stringify({
						content: content,
						group_id: $open_group_id,
					}),
				}
			)
			const data = await res.json()
			if (data.error) {
				alert(data.error)
			} else {
				get_group()
				send_message_unsetField('content')
			}
		}
	}

	const send_emoji = () => {
		send_message($open_group.group.emoji)
	}

	const {
		form: send_message_form,
		data: send_message_data,
		unsetField: send_message_unsetField,
	} = createForm({
		onSubmit: (values) => {
			send_message(values.content)
		},
	})

	let back_button

	open_group_id.subscribe((id) => {
		if (id) {
			get_group()
		}
	})

	let container

	let scrolled = false
	let bottomed = false
	const on_scroll = () => {
		scrolled = container.scrollTop > 0
		bottomed =
			container.scrollTop + container.clientHeight >= container.scrollHeight
	}

	const scroll_to_bottom_of_container = () => {
		container.scrollTop = container.scrollHeight
	}
</script>

{#if $open_group}
	<div
		class="group theme-{$open_group.group.theme}"
		transition:fly={{ x: '100%', opacity: 1 }}
		class:scrolled
		class:bottomed
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

		<div class="bottom-actions">
			<div class="message-input">
				<form use:send_message_form>
					<input type="text" placeholder="Aa" name="content" />
					<input
						type="submit"
						value={$_('send_message')}
						class="button black"
						disabled={!$send_message_data.content}
					/>
				</form>
				<div class="send-emoji">
					<a class="alt none" href="javascript:void(0);" on:click={send_emoji}>
						{$open_group.group.emoji}
					</a>
				</div>
			</div>
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
		margin-bottom: 3rem;
	}

	.bottom-actions {
		position: fixed;
		bottom: 0;
		width: 100%;
	}

	.message-input {
		float: right;
		margin-right: 1rem;
		background-color: var(--gray-200);
		transition: box-shadow 0.25s ease-in-out;
	}

	.group:not(.bottomed) .message-input {
		box-shadow: 0 0 2rem 2rem var(--gray-200);
	}

	.message-input input {
		display: inline-block;
	}

	.message-input input[type='submit'] {
		position: relative;
		top: -1px;
		margin-left: 0.5rem;
	}

	.message-input input[type='text'] {
		background-color: var(--gray-300);
	}

	.message-input input[type='text']:hover {
		background-color: var(--gray-100);
	}

	.message-input input[type='text']:active,
	.message-input input[type='text']:focus {
		background-color: var(--white);
	}

	.message-input input[type='submit'].button.black:disabled {
		background: var(--gray-300);
		color: var(--gray-400);
	}

	.message-input form,
	.send-emoji {
		display: inline-block;
	}

	.send-emoji {
		font-size: 2rem;
		margin-left: 1rem;
		position: relative;
		top: 0.25rem;
	}
</style>
