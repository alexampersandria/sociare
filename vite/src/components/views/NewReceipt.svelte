<script lang="ts">
	import { createForm } from 'felte'

	import { _ } from 'svelte-i18n'

	import { createEventDispatcher } from 'svelte'

	import { session } from '../../lib/stores/session'
	import { open_group, open_group_id } from '../../lib/stores/app'
	import CurrencyInput from '../CurrencyInput.svelte'

	const dispatch = createEventDispatcher()

	const {
		form: new_receipt_form,
		errors: new_receipt_errors,
		isValid: new_receipt_is_valid,
		data: new_receipt_data,
	} = createForm({
		onSubmit: async (values) => {
			const res = await fetch(
				`${import.meta.env.VITE_API_URL}/api/v1/receipt/create`,
				{
					method: 'POST',
					headers: {
						'Content-Type': 'application/json',
						Authorization: `Bearer ${$session}`,
					},
					body: JSON.stringify({
						group_id: $open_group_id,
						amount: raw_currency_value,
						info: values.info,
					}),
				}
			)
			const data = await res.json()
			if (data.error) {
				alert(data.error)
			} else {
				dispatch('receipt_created', data)
			}
		},
		validate: (value) => {
			const errors = {
				amount: '',
				info: '',
			}
			if (value.amount) {
				let only_0_to_9 = parseInt(value.amount.replace(/[^0-9]/g, ''))
				if (!Number.isInteger(only_0_to_9)) {
					errors.amount = $_('error_receipt_amount_not_integer')
				}
			}
			if (value.info) {
				if (value.info.length > 768) {
					errors.info = $_('error_receipt_info_too_long')
				}
			}
			return errors
		},
	})

	let raw_currency_value = 0
</script>

<form use:new_receipt_form class="wide">
	<label for="amount">{$_('amount_label')}</label>
	<CurrencyInput
		on:blur={(event) => {
			raw_currency_value = event.detail.raw
		}}
		name="amount"
		currency={$open_group.group.currency}
	/>
	{#if $new_receipt_errors.amount}
		<div class="error">{$new_receipt_errors.amount}</div>
	{/if}
	<label for="info">{$_('new_receipt_info_label')}</label>
	<input type="text" name="info" />
	{#if $new_receipt_errors.info}
		<div class="error">{$new_receipt_errors.info}</div>
	{/if}
	<input
		type="submit"
		value={$_('new_receipt_submit')}
		class="button pink"
		disabled={$new_receipt_is_valid && raw_currency_value > 0 ? false : true}
	/>
</form>
