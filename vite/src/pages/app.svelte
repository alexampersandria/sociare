<script lang="ts">
	import { _ } from 'svelte-i18n'

	import { goto } from '@roxi/routify'
	import {
		session_is_valid,
		session_fetch_completed,
		user_object,
	} from '../lib/stores/session'
	import GroupListings from '../components/views/GroupListings.svelte'
	import GroupView from '../components/views/GroupView.svelte'

	session_is_valid.subscribe((is_valid) => {
		if (!is_valid && session_fetch_completed) {
			$goto('/login')
		}
	})

	import { is_desktop } from '../lib/stores/app'
	import Spinner from '../components/Spinner.svelte'
	import Overlay from '../components/Overlay.svelte'
	import UserSettings from '../components/views/UserSettings.svelte'

	is_desktop.set(window.innerWidth >= 920)

	window.addEventListener('resize', () => {
		is_desktop.set(window.innerWidth >= 920)
	})

	let show_user_overlay = false
</script>

<svelte:head>
	<title>{$_('page_title')} &mdash; {$_('app_page_title')}</title>
</svelte:head>
{#if $user_object}
	<div class="app">
		<div class="left">
			<GroupListings
				on:user_settings={() => {
					show_user_overlay = true
				}}
			/>
		</div>
		<div class="right">
			<div class="no-group">
				<div class="no-group-message">{$_('no_group_selected')}</div>
			</div>
			<GroupView />
		</div>
		{#if show_user_overlay}
			<Overlay type="glass" show={show_user_overlay}>
				<UserSettings
					on:close={() => {
						show_user_overlay = false
					}}
				/>
			</Overlay>
		{/if}
	</div>
{:else}
	<div class="no-user-object">
		<Spinner center />
	</div>
{/if}

<style>
	.no-user-object {
		position: relative;
		inset: 0;
		height: 100vh;
		background: var(--gray-200);
		color: var(--gray-400);
	}

	.app {
		position: relative;
		overflow: hidden;
		width: 100vw;
		height: 100vh;
	}

	.no-group {
		display: none;
	}

	@media (min-width: 920px) {
		.app {
			display: flex;
		}

		.left,
		.right {
			position: relative;
			flex: 1;
		}

		.left {
			border-right: 2px solid var(--gray-300);
			max-width: min(33%, 32rem);
			min-width: 24rem;
		}

		:global(.left .group-listings .cover) {
			opacity: 0;
		}

		:global(.right .group-view .back) {
			display: none;
		}

		.no-group {
			display: block;
			position: relative;
			inset: 0;
			height: 100vh;
			background: var(--gray-200);
			color: var(--gray-400);
		}

		.no-group-message {
			position: absolute;
			top: 50%;
			left: 50%;
			translate: -50% -50%;
			white-space: nowrap;
		}
	}
</style>
