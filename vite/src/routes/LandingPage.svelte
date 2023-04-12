<script lang="ts">
	import { link } from 'svelte-spa-router'

	import '@/assets/app.scss'

	import payve_logo from '@/assets/payve_logo.svg'

	import { session_is_valid } from '../lib/stores'
</script>

<main>
	<section class="head" id="head">
		<div class="nav">
			<div class="container">
				<div class="logo">
					<img src={payve_logo} alt="payve_logo" />
				</div>
				<div class="links">
					<a href="/app" use:link class="alt none">
						{#if $session_is_valid}
							Go To App
						{:else}
							Log In
						{/if}
					</a>
				</div>
			</div>
		</div>

		<div class="call-to-action">
			<div class="container">
				<h1>
					House Share Economy<br />
					Made Easier
				</h1>
				<p class="explainer muted">
					Payve is an open source tool that allows groups of people to easily
					manage their finances and settle debts between them.
				</p>
				<div class="links">
					<a href="/docs" use:link class="button primary">Getting Started</a>
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
		<div class="container">
			<div class="placeholder">
				<div class="placeholder-image" />
				<div class="placeholder-text">
					<h2>Placeholder</h2>
					<p>
						Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem
						quis natus, saepe officia optio rerum ipsam sunt, voluptatum tempore
						doloremque exercitationem fuga doloribus culpa at vitae veniam
						consectetur eveniet error?
					</p>
				</div>
			</div>

			<div class="placeholder">
				<div class="placeholder-text align-right">
					<h2>Something Else</h2>
					<p>
						Lorem ipsum dolor sit amet consectetur adipisicing elit. Voluptatem
						quis natus, saepe officia optio rerum ipsam sunt, voluptatum tempore
						doloremque exercitationem fuga doloribus culpa at vitae veniam
						consectetur eveniet error?
					</p>
				</div>
				<div class="placeholder-image" />
			</div>
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
		border-bottom: #000 solid 2px;
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
		box-shadow: 0.5rem 0.5rem 0 #000;
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
