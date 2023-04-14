<script lang="ts">
	import { _ } from 'svelte-i18n'

	import payve_logo from '@/assets/payve_logo.svg'

	import { session_is_valid } from '../lib/stores'
	import Modal from '../components/Modal.svelte'
	import Login from '../components/Login.svelte'
	import Button from '../components/Button.svelte'
	import Spinner from '../components/Spinner.svelte'
	import { currency_codes, format_currency } from '../lib/econ'

	let show_login_modal = false

	let test_currency_format = 12345
</script>

<svelte:head>
	<title>{$_('page_title')} &mdash; {$_('landing_page_title')}</title>
</svelte:head>

<main>
	<Modal
		show={show_login_modal}
		on:close={() => {
			show_login_modal = false
		}}
		type="glass"
	>
		<Login />
	</Modal>

	<section class="head" id="head">
		<div class="nav">
			<div class="container">
				<div class="logo">
					<img src={payve_logo} alt="payve_logo" />
				</div>
				<div class="links">
					{#if $session_is_valid}
						<a href="/app" class="alt none">{$_('open_app')}</a>
					{:else}
						<a
							class="alt none"
							href="javascript:void(0)"
							on:click={() => {
								show_login_modal = true
							}}
						>
							{$_('log_in')}
						</a>
					{/if}
				</div>
			</div>
		</div>

		<div class="call-to-action">
			<div class="container">
				<h1>
					{$_('landing_head_title')}
				</h1>
				<p class="explainer muted">
					{$_('landing_head_subtitle')}
				</p>
				<div class="links">
					<a href="/docs" class="button black">{$_('getting_started')}</a>
					<a
						href="https://github.com/alexampersandria/sociare"
						target="_blank"
						class="button hollow">GitHub</a
					>
				</div>
			</div>
		</div>

		<div class="noise" />
	</section>

	<section>
		<div class="container" style="margin-top:2rem;">
			<h4>Buttons</h4>
			<Button variant="primary">Primary</Button>
			<Button variant="black">Black</Button>
			<Button variant="danger">Danger</Button>
			<Button variant="hollow">Hollow</Button>
		</div>
		<div class="container">
			<h4>Disabled</h4>
			<Button disabled variant="primary">Primary</Button>
			<Button disabled variant="black">Black</Button>
			<Button disabled variant="danger">Danger</Button>
			<Button disabled variant="hollow">Hollow</Button>
		</div>
		<div class="container">
			<h4>Spinner</h4>
			<Spinner />
		</div>
		<div class="container">
			<h4>Currency Format</h4>
			<input type="number" bind:value={test_currency_format} />
			{#each currency_codes as currency}
				<p>{format_currency(test_currency_format, currency)}</p>
			{/each}
		</div>
	</section>
</main>

<style lang="scss">
	main {
		min-height: 100vh;
		min-width: 100vw;
	}

	.head {
		position: relative;
		padding-bottom: 1rem;
		border-bottom: var(--black) solid 2px;
	}

	.head::before {
		inset: 0;
		content: '';
		display: block;
		position: absolute;
		z-index: -2;
		background-color: hsla(0, 0%, 100%, 1);
		background-image: radial-gradient(
				at 80% 61%,
				hsla(270, 47%, 72%, 1) 0px,
				transparent 50%
			),
			radial-gradient(at 32% 61%, hsla(33, 100%, 80%, 1) 0px, transparent 50%),
			radial-gradient(at 87% 20%, hsla(311, 98%, 75%, 1) 0px, transparent 50%),
			radial-gradient(at 51% 3%, hsla(337, 72%, 74%, 1) 0px, transparent 50%),
			radial-gradient(at 0% 0%, hsla(184, 50%, 52%, 1) 0px, transparent 50%);
		background-size: 200% 200%;
	}

	.head::after {
		inset: 0;
		content: '';
		display: block;
		position: absolute;
		z-index: -1;
		background-color: transparent;
		background-image: linear-gradient(transparent, #ffffff88),
			linear-gradient(60deg, #ffffff66, transparent, transparent),
			linear-gradient(-60deg, #ffffff66, transparent, transparent);
	}

	.nav,
	.call-to-action {
		position: relative;
		z-index: 1;
	}
	.nav .logo img {
		width: 100px;
		min-height: 40px;
	}

	.nav .container {
		display: flex;
		align-items: center;
		padding-top: 1rem;
	}

	.nav .container .links,
	.nav .container .logo {
		flex: 1;
	}

	.nav .links {
		text-align: right;
	}

	.call-to-action {
		text-align: center;
		padding: 2rem 0 6rem 0;
	}

	.call-to-action .explainer {
		max-width: 400px;
		margin: 0 auto;
		margin-bottom: 2rem;
	}

	.call-to-action .links > :global(.button) {
		margin: 0 0.5em;
	}

	.placeholder-image {
		position: relative;
		height: 10rem;
		background-color: #ccc;
		box-shadow: 0.5rem 0.5rem 0 var(--black);
		max-width: 20rem;
		margin: 0 2rem;
	}

	.placeholder-image::after {
		content: '';
		display: block;
		width: 5rem;
		height: 5rem;
		border-radius: 100%;
		position: absolute;
		inset: 50%;
		translate: -50% -50%;
		background: #aaa;
	}

	.placeholder {
		display: flex;
		padding: 3rem 0 0 0;
	}

	.placeholder-image,
	.placeholder-text {
		flex: 1 100%;
	}

	.placeholder-text h2 {
		margin: 0;
	}
</style>
