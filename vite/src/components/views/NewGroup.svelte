<script lang="ts">
	import { createForm } from 'felte'
	import { currency_codes } from '../../lib/econ'

	import { _ } from 'svelte-i18n'

	import { createEventDispatcher } from 'svelte'

	import { session } from '../../lib/stores/session'

	const dispatch = createEventDispatcher()

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
				dispatch('group_created', data)
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
