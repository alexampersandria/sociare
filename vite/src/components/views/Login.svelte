<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { createForm } from 'felte'
	import { log_in, sign_up } from '../../lib/auth'
	import { goto } from '@roxi/routify'
	import { validate } from 'email-validator'

	const username_regex = new RegExp('^[a-zA-Z0-9]+$')

	const {
		form: log_in_form,
		errors: log_in_errors,
		data: log_in_data,
		isValid: log_in_is_valid,
	} = createForm({
		onSubmit: async (value) => {
			if (value.username && value.password) {
				log_in(value.username, value.password).then(() => {
					$goto('/app')
				})
			}
		},
		validate: (value) => {
			const errors = {
				username: '',
				password: '',
			}
			if (value.username) {
				if (!username_regex.test(value.username)) {
					errors.username = $_('error_username_invalid')
				} else if (value.username.length < 3) {
					errors.username = $_('error_username_too_short')
				} else if (value.username.length > 24) {
					errors.username = $_('error_username_too_long')
				}
			}
			if (value.password) {
				if (value.password.length < 7) {
					errors.password = $_('error_password_too_short')
				} else if (value.password.length > 96) {
					errors.password = $_('error_password_too_long')
				}
			}
			return errors
		},
	})

	const {
		form: sign_up_form,
		errors: sign_up_errors,
		data: sign_up_data,
		isValid: sign_up_is_valid,
	} = createForm({
		onSubmit: async (value) => {
			if (value.username && value.password && value.email && value.name) {
				sign_up(value.username, value.password, value.email, value.name)
			}
		},
		validate: (value) => {
			const errors = {
				username: '',
				password: '',
				email: '',
				name: '',
			}
			if (value.username) {
				if (!username_regex.test(value.username)) {
					errors.username = $_('error_username_invalid')
				} else if (value.username.length < 3) {
					errors.username = $_('error_username_too_short')
				} else if (value.username.length > 24) {
					errors.username = $_('error_username_too_long')
				}
			}
			if (value.password) {
				if (value.password.length < 7) {
					errors.password = $_('error_password_too_short')
				} else if (value.password.length > 96) {
					errors.password = $_('error_password_too_long')
				}
			}
			if (value.email) {
				if (!validate(value.email)) {
					errors.email = $_('error_email_invalid')
				}
			}
			if (value.name) {
				if (value.name.length > 96) {
					errors.name = $_('error_display_name_too_long')
				}
			}
			return errors
		},
	})

	let action: 'login' | 'signup' = 'login'
</script>

<div class="login wide">
	{#if action === 'login'}
		<h3 class="align-center">{$_('log_in')}</h3>
		<form use:log_in_form>
			<label for="username">{$_('username_label')}</label>
			<input type="text" name="username" />
			{#if $log_in_errors.username}
				<div class="error">{$log_in_errors.username}</div>
			{/if}
			<label for="password">{$_('password_label')}</label>
			<input type="password" name="password" />
			{#if $log_in_errors.password}
				<div class="error">{$log_in_errors.password}</div>
			{/if}
			<input
				type="submit"
				value={$_('log_in_submit')}
				class="button pink"
				disabled={$log_in_is_valid &&
				$log_in_data.username &&
				$log_in_data.password
					? false
					: true}
			/>
		</form>
		<p class="muted align-center">
			{$_('sign_up_prompt')}
			<a href="#" on:click={() => (action = 'signup')}>{$_('sign_up')}</a>
		</p>
	{:else}
		<h3 class="align-center">{$_('sign_up')}</h3>
		<form use:sign_up_form>
			<label for="username">{$_('username_label')}</label>
			<input type="text" name="username" />
			{#if $sign_up_errors.username}
				<div class="error">{$sign_up_errors.username}</div>
			{/if}
			<label for="password">{$_('password_label')}</label>
			<input type="password" name="password" />
			{#if $sign_up_errors.password}
				<div class="error">{$sign_up_errors.password}</div>
			{/if}
			<label for="email">{$_('email_label')}</label>
			<input type="text" name="email" />
			{#if $sign_up_errors.email}
				<div class="error">{$sign_up_errors.email}</div>
			{/if}
			<label for="name">{$_('display_name_label')}</label>
			<input type="text" name="name" />
			{#if $sign_up_errors.name}
				<div class="error">{$sign_up_errors.name}</div>
			{/if}
			<input
				type="submit"
				value={$_('sign_up_submit')}
				class="button pink"
				disabled={$sign_up_is_valid &&
				$sign_up_data.username &&
				$sign_up_data.password &&
				$sign_up_data.email &&
				$sign_up_data.name
					? false
					: true}
			/>
		</form>
		<p class="muted align-center">
			{$_('log_in_prompt')}
			<a href="#" on:click={() => (action = 'login')}>{$_('log_in')}</a>
		</p>
	{/if}
</div>
