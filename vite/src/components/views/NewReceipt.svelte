<script lang="ts">
	import { createForm } from 'felte'

	import { _ } from 'svelte-i18n'

	import { createEventDispatcher } from 'svelte'

	import { session } from '../../lib/stores/session'
	import { open_group, open_group_id } from '../../lib/stores/app'
	import { format_currency } from '../../lib/econ'

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
						...values,
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
				if (!Number.isInteger(value.amount)) {
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
</script>

<form use:new_receipt_form class="wide">
	<label for="amount">{$_('amount_label')}</label>
	<input type="number" name="amount" />
	{#if $new_receipt_errors.amount}
		<div class="error">{$new_receipt_errors.amount}</div>
	{/if}
	{format_currency($new_receipt_data.amount, $open_group.group.currency)}
	<label for="info">{$_('new_receipt_info_label')}</label>
	<input type="text" name="info" />
	{#if $new_receipt_errors.info}
		<div class="error">{$new_receipt_errors.info}</div>
	{/if}
	<input
		type="submit"
		value={$_('new_receipt_submit')}
		class="button pink"
		disabled={$new_receipt_is_valid && $new_receipt_data.amount ? false : true}
	/>
</form>
