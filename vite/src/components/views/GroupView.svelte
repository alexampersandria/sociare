<script lang="ts">
	import { _ } from 'svelte-i18n'

	import {
		get_groups,
		is_desktop,
		open_group,
		open_group_id,
	} from '../../lib/stores/app'
	import { session } from '../../lib/stores/session'
	import { writable } from 'svelte/store'
	import { fly } from 'svelte/transition'
	import Message from '../Message.svelte'
	import GroupEvent from '../GroupEvent.svelte'

	import { format_currency } from '../../lib/econ'
	import { createForm } from 'felte'

	import { Receipt, ArrowLeft } from 'carbon-icons-svelte'
	import Overlay from '../Overlay.svelte'
	import Button from '../Button.svelte'
	import NewReceipt from './NewReceipt.svelte'
	import Modal from '../Modal.svelte'

	let container

	let scrolled = false
	let bottomed = false
	const on_scroll = () => {
		if (container) {
			scrolled = container.scrollTop > 0
			bottomed =
				container.scrollTop + container.clientHeight >= container.scrollHeight

			if (!scrolled) {
				get_more()
			}
		}
	}

	const scroll_to_bottom_of_container = () => {
		if (container) {
			container.scrollTop = container.scrollHeight
		}
	}

	const go_back = () => {
		open_group.set(null)
		open_group_id.set(null)
		scrolled = false
		bottomed = false
		can_load_more = true
	}
	const on_keydown = (event) => {
		if (event.key === 'Enter') {
			go_back()
		}
	}

	const limit = 32
	let can_load_more = true

	const group_fetch_completed = writable(false)
	const get_group = () => {
		if (open_group_id) {
			can_load_more = true
			group_fetch_completed.set(false)
			fetch(
				`${
					import.meta.env.VITE_API_URL
				}/api/v1/groups/${$open_group_id}?limit=${limit}`,
				{
					headers: {
						Authorization: `Bearer ${$session}`,
					},
				}
			)
				.then((res) => res.json())
				.then((data) => {
					if (data.error) {
						alert(data.error)
					} else {
						open_group.set(data[0])
						if (data[0].events) {
							if (data[0].events.length < limit) {
								can_load_more = false
							}
						}
					}
					group_fetch_completed.set(true)
				})
				.then(scroll_to_bottom_of_container)
		}
	}

	let container_height = 0
	const get_more = () => {
		if (open_group_id && can_load_more) {
			group_fetch_completed.set(false)
			fetch(
				`${
					import.meta.env.VITE_API_URL
				}/api/v1/groups/${$open_group_id}?limit=${limit}&offset=${
					$open_group.events.length
				}`,
				{
					headers: {
						Authorization: `Bearer ${$session}`,
					},
				}
			)
				.then((res) => res.json())
				.then((data) => {
					if (data.error) {
						alert(data.error)
					} else {
						container_height = container.scrollHeight
						let new_group = $open_group
						new_group.events = [...new_group.events, ...data[0].events]
						open_group.set(new_group)
						if (data[0].events.length < limit) {
							can_load_more = false
						}
					}
					group_fetch_completed.set(true)
				})
				.then(() => {
					container.scrollTop = container.scrollHeight - container_height
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
				get_groups($session)
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

	let show_receipt_overlay = false
</script>

{#if $open_group}
	<div
		class="group-view theme-{$open_group.group.theme}"
		transition:fly={{ x: $is_desktop ? '0%' : '100%', opacity: 1 }}
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

		<div class="sticky-bottom-wrapper">
			<div class="events">
				{#if !can_load_more}
					<div class="finished">{$_('reached_top_of_group')}</div>
				{/if}
				{#each $open_group.events.slice().reverse() as event}
					{#if event.message || event.receipt || event.transaction}
						<Message {event} />
					{:else}
						<GroupEvent {event} />
					{/if}
				{/each}
			</div>

			<div class="bottom-actions">
				<div class="message-input">
					<a
						href="javascript:void(0);"
						on:click={() => {
							show_receipt_overlay = true
						}}
						class="alt none receipt"
					>
						<Receipt size={32} />
					</a>
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
						<a
							class="alt none"
							href="javascript:void(0);"
							on:click={send_emoji}
						>
							{$open_group.group.emoji}
						</a>
					</div>
				</div>
			</div>
		</div>
	</div>
{/if}

<Modal
	type="glass"
	show={show_receipt_overlay}
	on:close={() => {
		show_receipt_overlay = false
	}}
>
	<NewReceipt
		on:receipt_created={() => {
			show_receipt_overlay = false
			get_group()
			get_groups($session)
		}}
	/>
</Modal>

<style>
	.group-view {
		position: absolute;
		inset: 0;
		background-color: var(--gray-200);
		height: 100vh;
		overflow-y: auto;
		z-index: 10;
	}

	.head {
		position: sticky;
		top: 0;
		background-color: var(--gray-200);
		transition: box-shadow 0.25s ease-in-out;
		padding: 1rem;
		z-index: 1;
	}

	.sticky-bottom-wrapper {
		position: absolute;
		bottom: 0;
		max-height: 100%;
		width: 100%;
	}

	.scrolled .head {
		box-shadow: 0 0 2rem 2rem var(--gray-200);
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
		margin-top: 1rem;
	}

	.bottom-actions {
		position: sticky;
		bottom: 0;
		background-color: var(--gray-200);
		display: flex;
		justify-content: space-between;
		padding-bottom: 1rem;
	}

	.message-input {
		float: right;
		margin-right: 1rem;
		background-color: var(--gray-200);
		transition: box-shadow 0.25s ease-in-out;
		display: flex;
		align-items: center;
		width: 100%;
		justify-content: space-between;
	}

	.group-view:not(.bottomed) .message-input {
		box-shadow: 0 0 1rem 1rem var(--gray-200);
	}

	.message-input .receipt {
		color: var(--gray-400);
		margin: 0.25rem 1rem 0 1rem;
	}

	.message-input .receipt:hover {
		color: var(--gray-500);
	}

	.message-input form {
		display: flex;
		align-items: center;
		width: 100%;
		flex-grow: 1;
	}

	.message-input input[type='submit'],
	.message-input input[type='text'] {
		margin: 0;
	}

	.message-input input[type='submit'] {
		margin-left: 0.5rem;
	}

	.message-input input[type='text'] {
		background-color: var(--gray-300);
		width: 100%;
		flex-grow: 1;
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

	.send-emoji {
		font-size: 2rem;
		margin-left: 1rem;
	}

	.finished {
		text-align: center;
		color: var(--gray-400);
		margin-bottom: 2rem;
	}
</style>
