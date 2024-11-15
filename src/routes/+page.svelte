<script>
	import Numberinput from '$lib/numberinput.svelte';
	import Plotly from 'plotly.js-dist';
	import proj4 from 'proj4';
	import { onMount } from 'svelte';

	const invoke = window.__TAURI__.core.invoke;
	let loading = false; // state to track loading status

	// Declarations
	let mapDiv;
	let parentDiv;
	let resizeObserver;
	const googlemaps = 'EPSG:4326';
	const utm32n = '+proj=utm +zone=32 +datum=WGS84 +units=m +no_defs';
	const utm33n = '+proj=utm +zone=33 +datum=WGS84 +units=m +no_defs';
	const tileURLTemplate =
		'https://cache.kartverket.no/topo/v1/wmts/1.0.0/default/googlemaps/{z}/{y}/{x}.png';

	// Initial state
	let xc = 596670;
	let yc = 6646968;
	let size = 250;
	let resolution = 5;
	$: numpoints = (size / resolution) ** 2;
	$: eta = numpoints / 8000;
	let selectedCRS = 'utm32n'; // Default coordinate system
	const [lons, lats] = proj4(utm32n, googlemaps, [xc, yc]);

	// Functions
	function latLonToXY(lat, lon, crs) {
		let projection;

		switch (crs) {
			case 'googlemaps':
				projection = googlemaps;
				break;
			case 'utm32n':
				projection = utm32n;
				break;
			case 'utm33n':
				projection = utm33n;
				break;
			default:
				throw new Error("Unsupported CRS. Choose 'googlemaps', 'utm32n', or 'utm33n'.");
		}

		const [x, y] = proj4(googlemaps, projection, [lon, lat]);
		return [x, y];
	}
	function xyToLatLon(x, y, crs) {
		let projection;
		switch (crs) {
			case 'googlemaps':
				projection = googlemaps;
				break;
			case 'utm32n':
				projection = utm32n;
				break;
			case 'utm33n':
				projection = utm33n;
				break;
			default:
				throw new Error('Unsupported CRS for UTM.');
		}
		const [lon, lat] = proj4(projection, googlemaps, [x, y]);
		return [lat, lon];
	}
	function createMap() {
		Plotly.newPlot(mapDiv, [], layout, config);
	}
	function updatePlotPoint() {
		const [plotLat, plotLon] = xyToLatLon(xc, yc, selectedCRS);
		const latLonCorners = [
			xyToLatLon(xc - size / 2, yc + size / 2, 'utm32n'),
			xyToLatLon(xc + size / 2, yc + size / 2, 'utm32n'),
			xyToLatLon(xc + size / 2, yc - size / 2, 'utm32n'),
			xyToLatLon(xc - size / 2, yc - size / 2, 'utm32n'),
			xyToLatLon(xc - size / 2, yc + size / 2, 'utm32n')
		];

		const trace = {
			type: 'scattermapbox',
			mode: 'lines',
			fill: 'toself',
			lat: latLonCorners.map((coord) => coord[0]),
			lon: latLonCorners.map((coord) => coord[1]),
			line: { color: 'red', width: 2 },
			fillcolor: 'rgba(255, 0, 0, 0.3)' // Transparent red fill
		};
		Plotly.react(mapDiv, [trace], layout, config);
	}
	// API
	async function test() {
		loading = true; // Start loading

		var filename = 'download.ifc';
		let file = await invoke('gen_ifc', {
			xc,
			yc,
			width: size,
			height: size,
			resolution,
			coordSys: 32632
		});
		var array = new Uint8Array(file);

		var string = new TextDecoder().decode(array);
		var blob = new Blob([string], { type: 'text/plain' });

		if (window.navigator && window.navigator.msSaveOrOpenBlob) {
		} else {
			var e = document.createEvent('MouseEvents'),
				a = document.createElement('a');
			a.download = filename;
			a.href = window.URL.createObjectURL(blob);
			a.dataset.downloadurl = ['text/plain', a.download, a.href].join(':');
			e.initEvent('click', true, false, window, 0, 0, 0, 0, 0, false, false, false, false, 0, null);
			a.dispatchEvent(e);
		}
		loading = false; // End loading
	}
	function test2() {
		const mapCenter = layout.mapbox.center;
		const centerLat = mapCenter.lat;
		const centerLon = mapCenter.lon;

		// Convert the map center from lat/lon to the selected CRS (e.g., UTM)
		const [newX, newY] = latLonToXY(centerLat, centerLon, selectedCRS);

		// Update the rectangle's center coordinates
		xc = newX;
		yc = newY;

		onCoordinateChange();
	}

	// Config
	const layout = {
		mapbox: {
			style: 'white-bg',
			center: { lat: lats, lon: lons },
			zoom: 16,
			layers: [
				{
					sourcetype: 'raster',
					source: [tileURLTemplate],
					below: 'traces'
				}
			]
		},
		margin: { t: 0, r: 0, b: 0, l: 0 },
		showlegend: false,
		clickmode: 'event',
		dragmode: 'zoom',
		hovermode: 'closest'
	};
	const config = { scrollZoom: true };

	// Event handlers
	function onCoordinateChange() {
		updatePlotPoint();
	}

	// Initialization
	onMount(() => {
		createMap();

		updatePlotPoint(); // Observe changes in parent container size
		const parentDiv = mapDiv.parentElement;
		resizeObserver = new ResizeObserver(() => {
			Plotly.Plots.resize(mapDiv);
		});
		resizeObserver.observe(parentDiv);
	});
</script>

<div id="header"></div>
<div id="map-container" bind:this={parentDiv}>
	<!-- Sidebar for CRS Selector -->

	<!-- Map Container -->
	<div id="map" bind:this={mapDiv}></div>
</div>
<div id="controls">
	Kartinnstillinger

	<select id="crs-select" bind:value={selectedCRS}>
		<option value="googlemaps">EPSG:3857 (Web Mercator)</option>
		<option value="utm32n">UTM Zone 32N</option>
		<option value="utm33n">UTM Zone 33N</option>
	</select>
	<span>
		<Numberinput label="X (m)" bind:value={xc} change={onCoordinateChange} />
		<Numberinput label="Y (m)" bind:value={yc} change={onCoordinateChange} />
	</span>
	<span>
		<Numberinput label="Størrelse (m)" bind:value={size} change={onCoordinateChange} />
		<Numberinput label="Oppløsning (m)" bind:value={resolution} change={null} />
	</span>

	<p>{numpoints.toFixed(0)} punkter / est. {eta.toFixed(1)}s</p>
	<span
		><button on:click={test2}>Sentrer kart</button>
		<button on:click={test}>
			{#if loading}
				<div class="spinner"></div>
			{:else}
				Generer
			{/if}</button
		>
	</span>
</div>

<style>
	/* Prevent scrolling on the whole page */
	#header {
		width: 100%;
		background-color: #5e5e5e;
		height: 50px;
		box-shadow:
			rgba(0, 0, 0, 0.12) 0px 1px 10px 0px,
			rgba(0, 0, 0, 0.14) 0px 4px 5px 0px,
			rgba(0, 0, 0, 0.2) 0px 2px 4px -1px;
		box-sizing: border-box;
		position: fixed;
		z-index: 1000;
	}
	#map-container {
		top: 50px;

		display: flex;
		position: absolute;
		width: 100%;
		height: 100%; /* Fill the entire height of the page */
	}

	#map {
		width: 100%;
		height: 100%; /* Ensure the map takes up the full height */
	}
	span {
		display: flex;
		flex-direction: row;
		justify-content: space-between;
	}

	#controls {
		left: 10px;
		top: 60px;
		position: relative;
		border-radius: 5px;
		z-index: 2;
		width: 200px;
		padding: 10px;
		background-color: #ffffff;
		height: 100%; /* Ensure sidebar takes up full height */
	}
	p {
		font-size: 0.5em;
		display: inline-block;
		width: 96%;
		text-align: end;
	}

	button {
		background-color: #0073d1;
		color: white;
		border: 0px;
		border-radius: 4px;
		padding: 0.5em;
		transition-duration: 0.25s;
		display: flex;
		flex: 1;
		flex-direction: row;
		justify-content: center;
	}
	button:hover {
		background-color: #005192;
		cursor: pointer;
		transition-duration: 0.25s;
	}

	span {
		flex-direction: row;
		justify-content: space-between;
		gap: 10px; /* Add a gap between the buttons */
	}
	.spinner {
		border: 4px solid #f3f3f3; /* Light grey */
		border-top: 4px solid #3498db; /* Blue */
		border-radius: 50%;
		width: 10px;
		height: 10px;
		animation: spin 2s linear infinite;
		position: absolute;
	}

	/* Keyframe for spinner animation */
	@keyframes spin {
		0% {
			transform: rotate(0deg);
		}
		100% {
			transform: rotate(360deg);
		}
	}
</style>
