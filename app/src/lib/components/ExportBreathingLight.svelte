<!-- <div class="breathing-light"></div> -->
<div class="flex flex-row items-center p-2 text-slate-400">
	<span>Exporting</span>
	<div class="breathing-light"></div>
</div>

<style>
	.breathing-light {
		width: 10px;
		height: 10px;
		background-color: #19f07a;
		border-radius: 50%;
		animation: breathing 2.5s infinite;
		margin-left: 10px;
	}

	@keyframes breathing {
		0%,
		100% {
			transform: scale(1);
			opacity: 1;
		}
		50% {
			transform: scale(1.1);
			opacity: 0.2;
		}
	}
</style>
