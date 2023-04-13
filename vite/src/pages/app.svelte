<script>
	import { goto } from '@roxi/routify'
	import Button from '../components/Button.svelte'
	import {
		user_object,
		session,
		session_is_valid,
		session_fetch_completed,
	} from '../lib/stores'

	import { log_out } from '../lib/auth'
	import Spinner from '../components/Spinner.svelte'

	session_is_valid.subscribe((is_valid) => {
		if (!is_valid && session_fetch_completed) {
			$goto('/login')
		}
	})
</script>

<div class="container">
	{#if $session_is_valid && $user_object && $session && session_fetch_completed}
		<div class="user">
			<h2>user</h2>
			<div class="id">
				<strong>id: </strong>
				{$user_object.id}
			</div>
			<div class="username">
				<strong>username: </strong>
				{$user_object.username}
			</div>
			<div class="name">
				<strong>name: </strong>
				{$user_object.name}
			</div>
		</div>
		<div class="session">
			<h2>Authorization</h2>
			<div class="id">
				<strong>Bearer </strong>
				{$session}
			</div>
		</div>
		<div class="actions" style="margin-top: 2rem;">
			<Button variant="danger" on:click={log_out}>Log Out</Button>
		</div>
	{/if}
</div>
