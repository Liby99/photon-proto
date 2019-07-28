const Photon = require("../../lib/index");

const canvas = document.getElementById("main-canvas");
const { width, height } = canvas;
const context = canvas.getContext("2d");
const imgData = context.createImageData(width, height);
const imgBuffer = imgData.data;

Photon.render({}, imgBuffer);

context.putImageData(imgData, 0, 0);