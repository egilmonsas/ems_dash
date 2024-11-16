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
	let showPanel = false;

	const tileURLTemplate =
		'https://cache.kartverket.no/topo/v1/wmts/1.0.0/default/googlemaps/{z}/{y}/{x}.png';

	const crsConfig = {
		googlemaps: { proj: 'EPSG:4326', sys: 4326 },
		'UTM 32N': { proj: '+proj=utm +zone=32 +datum=WGS84 +units=m +no_defs', sys: 32632 },
		'UTM 33N': { proj: '+proj=utm +zone=33 +datum=WGS84 +units=m +no_defs', sys: 32633 },
		'UTM 35N': { proj: '+proj=utm +zone=35 +datum=WGS84 +units=m +no_defs', sys: 32633 },
		'NTM 5': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=5.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5105
		},
		'NTM 6': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=6.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5106
		},
		'NTM 7': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=7.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5107
		},
		'NTM 8': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=8.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5108
		},
		'NTM 9': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=9.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5109
		},
		'NTM 10': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=10.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5110
		},
		'NTM 11': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=11.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5111
		},
		'NTM 12': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=12.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5112
		},
		'NTM 13': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=13.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5113
		},
		'NTM 14': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=14.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5114
		},
		'NTM 15': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=15.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5115
		},
		'NTM 16': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=16.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5116
		},
		'NTM 17': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=17.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5117
		},
		'NTM 18': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=18.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 19': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=19.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 20': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=20.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 21': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=21.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 22': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=22.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 23': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=23.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 24': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=24.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 25': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=25.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 26': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=26.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 27': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=27.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 28': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=28.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 29': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=29.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		},
		'NTM 30': {
			proj: '+proj=tmerc +lat_0=58 +lon_0=30.5 +x_0=100000 +y_0=1000000 +ellps=GRS80 +units=m +no_defs',
			sys: 5118
		}
	};
	// Initial state
	let xc = 596670;
	let yc = 6646968;
	let size = 250;
	let resolution = 5;
	$: numpoints = (size / resolution) ** 2;
	$: eta = numpoints / 8000;
	let selectedCRS = 'UTM 32N';
	let baseCRS = 'googlemaps';
	let previousCRS = selectedCRS;

	const [lats, lons] = xyToLatLon(xc, yc, selectedCRS);

	// Functions
	function latLonToXY(lat, lon, crs) {
		const [x, y] = proj4(crsConfig[baseCRS].proj, crsConfig[crs].proj, [lon, lat]);
		return [x, y];
	}
	function xyToLatLon(x, y, crs) {
		const [lon, lat] = proj4(crsConfig[crs].proj, crsConfig[baseCRS].proj, [x, y]);
		return [lat, lon];
	}
	function createMap() {
		Plotly.newPlot(mapDiv, [], layout, config);
	}
	function updatePlotPoint() {
		const latLonCorners = [
			xyToLatLon(xc - size / 2, yc + size / 2, selectedCRS),
			xyToLatLon(xc + size / 2, yc + size / 2, selectedCRS),
			xyToLatLon(xc + size / 2, yc - size / 2, selectedCRS),
			xyToLatLon(xc - size / 2, yc - size / 2, selectedCRS),
			xyToLatLon(xc - size / 2, yc + size / 2, selectedCRS)
		];
		console.log(latLonCorners);
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
	async function generateIFC() {
		loading = true; // Start loading

		var filename = 'download.ifc';
		console.log(crsConfig[baseCRS].sys);
		let file = await invoke('gen_ifc', {
			xc,
			yc,
			width: size,
			height: size,
			resolution,
			coordSys: crsConfig[selectedCRS].sys
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
	function moveRectangle() {
		const mapCenter = layout.mapbox.center;
		const centerLat = mapCenter.lat;
		const centerLon = mapCenter.lon;

		// Convert the map center from lat/lon to the selected CRS (e.g., UTM)
		const [newX, newY] = latLonToXY(centerLat, centerLon, selectedCRS);

		// Update the rectangle's center coordinates
		xc = Math.round(newX);
		yc = Math.round(newY);

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
	// Event handlers
	function handlePaste(event) {
		event.preventDefault();
		const pastedData = event.clipboardData.getData('Text');
		const rows = pastedData.split('\n');

		Coordinates = rows.map((row) => {
			const cols = row.split('\t');
			return {
				x: cols[0] || '',
				y: cols[1] || ''
			};
		});
	}
	function onPanelClick() {
		showPanel = !showPanel;
	}
	function onCRSChange() {
		const [plotLat, plotLon] = xyToLatLon(xc, yc, previousCRS);
		[xc, yc] = latLonToXY(plotLat, plotLon, selectedCRS);
		xc = Math.round(xc);
		yc = Math.round(yc);
		updatePlotPoint();
		previousCRS = selectedCRS;
	}

	function onCRS2Change() {
		transformCoordinates();
		previousCRS2 = selectedCRS2;
	}
	let Coordinates = [{ x: '596670', y: '6646968' }];
	let selectedCRS2 = 'UTM 32N';
	let previousCRS2 = 'UTM 32N';
	function transformCoordinates() {
		const precision = 1000;
		Coordinates = Coordinates.map((coord) => {
			try {
				const projectionInput = crsConfig[previousCRS2].proj;
				const projectionOutput = crsConfig[selectedCRS2].proj;
				const [transformedX, transformedY, transformedZ] = proj4(
					projectionInput,
					projectionOutput,
					[parseFloat(coord.x), parseFloat(coord.y)]
				);
				return {
					x: Math.round(transformedX * precision) / precision,
					y: Math.round(transformedY * precision) / precision
				};
			} catch (e) {
				console.error('Transformation error: ', e);
				return { x: '', y: '' };
			}
		});
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

<div class="panel" hidden={!showPanel}>
	<h3>Coordinate Transformation</h3>
	<select id="crs-select" bind:value={selectedCRS2} on:change={onCRS2Change}>
		{#each Object.entries(crsConfig).slice(1) as [key, label]}
			<option value={key}>{key}</option>
		{/each}
	</select>
	<table on:paste={handlePaste}>
		<thead>
			<tr>
				<th>X (Longitude/Easting)</th>
				<th>Y (Latitude/Northing)</th>
			</tr>
		</thead>
		<tbody>
			{#each Coordinates as coord, index}
				<tr>
					<td>{coord.x}</td>
					<td>{coord.y}</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
<div id="header"></div>
<div id="map-container" bind:this={parentDiv}>
	<!-- Sidebar for CRS Selector -->

	<!-- Map Container -->
	<div id="map" bind:this={mapDiv}></div>
</div>
<div id="controls">
	Kartinnstillinger

	<select id="crs-select" bind:value={selectedCRS} on:change={onCRSChange}>
		{#each Object.entries(crsConfig).slice(1) as [key, label]}
			<option value={key}>{key}</option>
		{/each}
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
		><button on:click={moveRectangle}>Sentrer kart</button>
		<button on:click={generateIFC}>
			{#if loading}
				<div class="spinner"></div>
			{:else}
				Generer
			{/if}</button
		>
	</span>
	<button class="btn" on:click={onPanelClick}>Transform Coordinates</button>
</div>

<style>
	.panel {
		position: absolute;
		top: 60px;
		left: 240px;
		background: #fff;
		border: 1px solid #ccc;
		padding: 15px;
		width: 400px;
		box-shadow: 0px 4px 10px rgba(0, 0, 0, 0.1);
		z-index: 10;
		border-radius: 5px;
	}
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
		height: 100%;
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
		width: 100%;
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
