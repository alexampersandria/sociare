<script>
	import { _, locale } from 'svelte-i18n'
	import { createEventDispatcher } from 'svelte'
	import { user_object } from '../../lib/stores/session'
	import moment from 'moment'

	import Button from '../Button.svelte'
	import { gravatar } from '../../lib/gravatar'
	import { time_since } from '../../lib/stores/app'

	import { Close } from 'carbon-icons-svelte'
	import { log_out } from '../../lib/auth'

	const dispatch = createEventDispatcher()

	const close = () => {
		dispatch('close')
	}
</script>

<div class="close">
	<a href="javascript:void(0);" on:click={close} class="alt none">
		<Close size={24} />
	</a>
</div>

<div class="container s">
	{#if $user_object}
		<div class="user-info">
			<div class="image align-center">
				<img
					class="round"
					src={gravatar($user_object.email, 512)}
					alt="Gravatar"
				/>
				<p class="muted">
					Change your avatar using <a
						target="_blank"
						href="https://en.gravatar.com/">Gravatar</a
					>.
				</p>
			</div>
			<div class="user-table-wrapper">
				<table class="user-table">
					<tr>
						<td class="field">{$_('username_label')}</td>
						<td class="value">@{$user_object.username}</td>
					</tr>
					<tr>
						<td class="field">{$_('email_label')}</td>
						<td class="value">{$user_object.email}</td>
					</tr>
					<tr>
						<td class="field">{$_('display_name_label')}</td>
						<td class="value">{$user_object.name}</td>
					</tr>
				</table>
			</div>
			<div class="user-since align-center">
				<p class="muted">
					{$_('user_since_label')}
					{moment($user_object.created_at).format('YYYY-MM-DD')} ({time_since(
						$user_object.created_at,
						$locale,
						true
					)})
				</p>
			</div>
			<div class="actions align-center">
				<Button variant="danger" on:click={log_out}>
					{$_('logout_label')}
				</Button>
			</div>
		</div>
	{/if}
</div>

<style>
	.user-info .image img {
		width: calc(100% - 4rem);
		max-width: 256px;
	}

	.container {
		margin-top: 4rem;
	}

	.close {
		position: absolute;
		top: 1rem;
		left: 1rem;
	}

	.user-table-wrapper {
		display: flex;
		justify-content: center;
	}

	.user-table {
		margin-top: 2rem;
	}

	.user-table .field {
		padding-right: 1rem;
		font-weight: 700;
	}
</style>
