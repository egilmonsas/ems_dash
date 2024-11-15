<script lang="ts">
	export let label;
	export let value: number;
	export let change: any;
</script>

<div>
	<label id="inputlabel" for={label}>{label}</label>
	<div>
		<input class="inputfield" type="number" id={label} bind:value on:change={change} />
	</div>
</div>

<style>
	.inputfield {
		padding-left: 7px;
	}
	#inputlabel {
		position: relative;
		z-index: 3;
		background-color: white;
		top: 8px;
		left: 4px;
		padding-left: 5px;
		padding-right: 5px;
		font-size: 0.5rem;
		color: #aaaaaa;
	}

	input {
		box-sizing: border-box;
		cursor: text;
		display: inline-flex;
		-webkit-box-align: center;
		align-items: center;
		width: 100%;
		height: 2em;
		font-family: 'Open Sans', Arial, sans-serif;
		line-height: 1.4375em;
		font-size: 0.75rem;
		font-weight: 400;
		color: rgba(0, 0, 0, 0.8);

		position: relative;

		border-radius: 4px;
		border-width: 0.5px;
		border-color: #aaaaaa;
	}
	input:focus {
		outline: 1px solid blue;
	}
</style>
