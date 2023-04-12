<script lang="ts">
	import "./assets/app.scss";

	import payve_logo from "./assets/payve_logo.svg";

	import yen_banknote_3d from "./assets/fluent_emoji/yen_banknote_3d.png";
	import euro_banknote_3d from "./assets/fluent_emoji/euro_banknote_3d.png";
	import coin_3d from "./assets/fluent_emoji/coin_3d.png";
	import dollar_banknote_3d from "./assets/fluent_emoji/dollar_banknote_3d.png";
</script>

<main>
	<div class="head" id="head">
		<div class="floating-emojis">
			<div class="left side">
				<div class="emoji emoji-1">
					<div class="yen">
						<img src={yen_banknote_3d} alt="Yen Emoji" />
					</div>
				</div>
				<div class="emoji emoji-2">
					<div class="euro">
						<img src={euro_banknote_3d} alt="Euro Emoji" />
					</div>
				</div>
			</div>
			<div class="right side">
				<div class="emoji emoji-3">
					<div class="coin">
						<img src={coin_3d} alt="Coin Emoji" />
					</div>
				</div>
				<div class="emoji emoji-4">
					<div class="dollar">
						<img src={dollar_banknote_3d} alt="Dollar Emoji" />
					</div>
				</div>
			</div>
		</div>

		<div class="nav">
			<div class="container">
				<div class="logo">
					<img src={payve_logo} alt="payve_logo" />
				</div>
				<div class="links">
					<a href="/app" class="alt none">Log In</a>
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
					<a href="/docs" class="button primary">Getting Started</a>
					<a
						href="https://github.com/alexampersandria/sociare"
						target="_blank"
						class="button hollow">GitHub</a
					>
				</div>
			</div>
		</div>
	</div>
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
		content: "";
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
		content: "";
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

	.container {
		max-width: 960px;
		margin: 0 auto;
		padding: 0 1rem;
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

	.call-to-action .links > :global(*) {
		margin: 0 0.5em;
	}

	.floating-emojis {
		max-width: 960px;
		margin: 0 auto;
		position: absolute;
		inset: 0;
		pointer-events: none;
	}

	.floating-emojis .side {
		position: absolute;
		width: 100%;
		height: 100%;
	}

	.floating-emojis .right.side .emoji {
		right: 0;
	}

	.floating-emojis .emoji {
		position: absolute;
		top: 50%;
		translate: 0 -50%;
    animation: bob 6s cubic-bezier(0.5, 0.05, 0.5, 0.95) infinite;
	}

  @for $i from 1 through 4 {
    .floating-emojis .emoji.emoji-#{$i} {
      animation-delay: -#{$i*1.5}s;
    }
  }

  @keyframes bob {
    0%, 100% {
      transform: translate(0, 0);
    } 50% {
      transform: translate(0, -1rem);
    }
  }

	.floating-emojis .emoji img {
		width: min(12rem, 20vw);
	}

	.floating-emojis .yen {
		transform: rotate(-12.5deg) translate(-6rem, -5rem);
		filter: drop-shadow(-0.5rem -0.5rem 3rem #a7612c66);
	}

	.floating-emojis .yen img {
		max-width: 11rem;
	}

	.floating-emojis .euro {
		transform: rotate(25deg) translate(1rem, 1rem);
		filter: drop-shadow(-0.5rem -0.5rem 3rem #17749d66);
	}

	.floating-emojis .coin {
		transform: rotate(12.5deg) translate(-2rem, -2rem);
		filter: drop-shadow(-1rem 0 3rem #be824366);
	}

	.floating-emojis .coin img {
		max-width: 11rem;
	}

	.floating-emojis .dollar {
		transform: rotate(-17.5deg) translate(6rem, 3rem);
		filter: drop-shadow(-1rem 0 3rem #1e906166);
	}
</style>
