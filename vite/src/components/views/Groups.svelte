<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { session } from '../../lib/stores'

	import { writable } from 'svelte/store'
	import Button from '../Button.svelte'
	import Modal from '../Modal.svelte'
	import { createForm } from 'felte'
	import { currency_codes, format_currency } from '../../lib/econ'

	const groups = writable([])
	const groups_fetch_completed = writable(false)

	const get_groups = () => {
		groups_fetch_completed.set(false)
		fetch(`${import.meta.env.VITE_API_URL}/api/v1/groups`, {
			headers: {
				Authorization: `Bearer ${$session}`,
			},
		})
			.then((res) => res.json())
			.then((data) => {
				if (data.error) {
					alert(data.error)
				} else {
					groups.set(data)
				}
				groups_fetch_completed.set(true)
			})
	}

	get_groups()

	let show_new_group_modal = false

	const {
		form: new_group_form,
		errors: new_group_errors,
		isValid: new_group_is_valid,
		data: new_group_data,
	} = createForm({
		onSubmit: async (values) => {
			const res = await fetch(
				`${import.meta.env.VITE_API_URL}/api/v1/group/create`,
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Authorization: `Bearer ${$session}`,
					},
					body: JSON.stringify(values),
				}
			)
			const data = await res.json()
			if (data.error) {
				alert(data.error)
			} else {
				get_groups()
				show_new_group_modal = false
			}
		},
		validate: (value) => {
			const errors = {
				name: '',
				currency: '',
			}
			if (value.name) {
				if (value.name.length > 48) {
					errors.name = $_('error_group_name_too_long')
				}
			}
			if (value.currency) {
				if (value.currency.length !== 3) {
					errors.currency = $_('error_group_currency_invalid_format')
				} else if (currency_codes.indexOf(value.currency) === -1) {
					errors.currency = $_('error_group_currency_not_supported')
				}
			}
			return errors
		},
	})
</script>

<Modal
	show={show_new_group_modal}
	type="glass"
	on:close={() => {
		show_new_group_modal = false
	}}
>
	<form use:new_group_form class="wide">
		<label for="name">{$_('new_group_name_label')}</label>
		<input type="text" name="name" />
		{#if $new_group_errors.name}
			<div class="error">{$new_group_errors.name}</div>
		{/if}
		<label for="currency">{$_('new_group_currency_label')}</label>
		<input type="text" name="currency" />
		{#if $new_group_errors.currency}
			<div class="error">{$new_group_errors.currency}</div>
		{/if}
		<input
			type="submit"
			value={$_('new_group_submit')}
			class="button pink"
			disabled={$new_group_is_valid &&
			$new_group_data.name &&
			$new_group_data.currency
				? false
				: true}
		/>
	</form>
</Modal>

<div class="group-container">
	<div class="head">
		<h2>{$_('groups')}</h2>
		<Button
			on:click={() => {
				show_new_group_modal = true
			}}>{$_('new_group_button')}</Button
		>
	</div>
	{#if $groups.length > 0}
		{#each $groups as group}
			<div class="group">
				<h2>{group.group.name}</h2>
				<div>{format_currency(group.group.total, group.group.currency)}</div>
				<pre><code>{JSON.stringify(group, undefined, 2)}</code></pre>
			</div>
		{/each}
	{:else if !$groups_fetch_completed}
		<div class="loading">Loading...</div>
	{:else}
		<div class="no_groups_found">{$_('no_groups_found')}</div>
	{/if}
</div>

<style>
	.group-container {
		display: relative;
		height: 100vh;
		overflow-y: scroll;
		background-color: var(--gray-200);
		min-width: 10rem;
		flex: 1;
	}

	.loading,
	.no_groups_found {
		position: absolute;
		top: 50%;
		left: 50%;
		translate: -50% -50%;
		white-space: nowrap;
		color: var(--gray-400);
	}
</style>
