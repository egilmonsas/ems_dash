<script>
    import { onMount } from "svelte";
    import Plotly from "plotly.js-dist";
    import proj4 from 'proj4';

    let mapDiv;
    let selectedCRS = "utm32n"; // Default coordinate system
  
    // Base URL template with placeholders for {TileMatrixSet}
    const tileURLTemplate = "https://cache.kartverket.no/topo/v1/wmts/1.0.0/default/{TileMatrixSet}/{z}/{y}/{x}.png";
  
    // Function to get the tile URL based on the selected CRS
    function getTileURL(crs) {
     
        return tileURLTemplate.replace("{TileMatrixSet}", crs);
    }
    function getZoomLevel(crs) {
    switch (crs) {
      case "googlemaps":
        return 6;  // Suitable zoom for global projection (Web Mercator)
      case "utm32n":
        return 5;  // Appropriate zoom for local UTM zone 32N (Norway)
      case "utm33n":
        return 5;  // Appropriate zoom for local UTM zone 33N (Norway)
      default:
        return 6;
    }
  }
  let center = { lat: 60.472, lon: 8.4689 }; // Default to the geographic center of Norway
  
  function latLonToXY(lat, lon, crs) {
  let projection;

  // Default to WGS84 for lat/lon
  let transformed;

  switch (crs) {
    case "googlemaps": // Web Mercator (EPSG:3857)
      projection = "EPSG:3857"; // Web Mercator projection
      transformed = proj4("WGS84", "EPSG:3857", [lon, lat]);  // Lat/Lon -> Web Mercator (x, y in meters)
      break;

    case "utm32n": // UTM Zone 32N (Norway)
      // UTM Zone 32N projection, converting from Lat/Lon (WGS84) to UTM
      projection = "+proj=utm +zone=32 +datum=WGS84 +units=m +no_defs"; // UTM 32N
      transformed = proj4("WGS84", projection, [lon, lat]); // Lat/Lon -> UTM (meters)
      break;

    case "utm33n": // UTM Zone 33N (Norway)
      // UTM Zone 33N projection, converting from Lat/Lon (WGS84) to UTM
      projection = "+proj=utm +zone=33 +datum=WGS84 +units=m +no_defs"; // UTM 33N
      transformed = proj4("WGS84", projection, [lon, lat]); // Lat/Lon -> UTM (meters)
      break;

    default:
      transformed = proj4("WGS84", "EPSG:3857", [lon, lat]);
      break;
  }

  // Log x and y values for debugging
  console.log(`Transformed Coordinates [${crs}]:`, { x: transformed[0], y: transformed[1] });

  return transformed; // Returns the transformed [x, y] coordinates
}

    // Plotly map setup function
    function createMap() {
        const zoom = getZoomLevel(selectedCRS);

        const [x, y] = latLonToXY(center.lat, center.lon, selectedCRS);

      const data = [
        {
          type: "scattermapbox",
          x: [x],  // Plot in local x (projected coordinates)
          y: [y],  // Plot in local y (projected coordinates)
          mode: "markers+text",
          text: ["Norway Center"],
          marker: { size: 10, color: "blue" },
        },
      ];
  
      const layout = {
        mapbox: {
          style: "white-bg",
          center: { lat: 60.472, lon: 8.4689 },
          zoom: 5,
          layers: [
            {
              sourcetype: "raster",
              source: [getTileURL(selectedCRS)],
              below: "traces",
            },
          ],
        },
        margin: { t: 0, r: 0, b: 0, l: 0 },
      };
  
      Plotly.newPlot(mapDiv, data, layout);
    }
  
    // Update map when CRS changes
    function updateMap() {
        const zoom = getZoomLevel(selectedCRS);
        const [x, y] = latLonToXY(center.lat, center.lon, selectedCRS);

      const newSource = [getTileURL(selectedCRS)];
      Plotly.relayout(mapDiv, "mapbox.layers[0].source", newSource);
        Plotly.relayout(mapDiv, {
        "mapbox.zoom": zoom,          
        "mapbox.center": { lat: 60.472, lon: 8.4689 },
        });
    }
  
    // Initialize the map on mount
    onMount(() => {
      createMap();
    });
  </script>
  
  <style>
    #map-container {
      display: flex;
    }
    #map {
      width: 80%;
      height: 100vh;
    }
    #controls {
      width: 20%;
      padding: 10px;
      background-color: #f5f5f5;
    }
  </style>
  
  <div id="map-container">
    <!-- Map Container -->
    <div id="map" bind:this={mapDiv}></div>
  
    <!-- Sidebar for CRS Selector -->
    <div id="controls">
      <label for="crs-select">Select Coordinate System:</label>
      <select id="crs-select" bind:value={selectedCRS} on:change={updateMap}>
        <option value="googlemaps">EPSG:3857 (Web Mercator)</option>
        <option value="utm32n">UTM Zone 32N</option>
        <option value="utm33n">UTM Zone 33N</option>
      </select>
    </div>
  </div>
  