const Photon = require("../../lib/index");
const $ = require("jquery");

// First initialize the event callbacks
let MAX_INCLINE = Math.PI / 2 - 0.01, MIN_INCLINE = -MAX_INCLINE;
let mouseDown = false;
let currX = 0.0, currY = 0.0;
let azimuth = 0.0, incline = 0.0;

const $canvas = $("#main-canvas");

$canvas.mousedown((event) => {
  mouseDown = true;
  currX = event.screenX;
  currY = event.screenY;
});

$canvas.mousemove((event) => {
  if (mouseDown) {
    const nextX = event.screenX;
    const nextY = event.screenY;

    const diffX = nextX - currX;
    const diffY = nextY - currY;

    // Calculate new azimuth and incline
    azimuth += diffX * 0.01;
    incline = Math.max(Math.min(incline + diffY * 0.01, MAX_INCLINE), MIN_INCLINE);

    currX = nextX;
    currY = nextY;
  }
});

$canvas.mouseup(() => {
  mouseDown = false;
});

// Then enter render loop
const canvas = document.getElementById("main-canvas");
const { width, height } = canvas;
const context = canvas.getContext("2d");
const imgData = context.createImageData(width, height);

setInterval(() => {

  // Update camera data
  Photon.mainCamera.azimuth = azimuth;
  Photon.mainCamera.incline = incline;

  // Render the scene and put it on the context
  Photon.render(imgData);
  context.putImageData(imgData, 0, 0);
}, 250); // Render the screen every 5 secs;