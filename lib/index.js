var addon = require('../native');

module.exports = {
  mainCamera: {
    target: {
      x: 0.0,
      y: 0.0,
      z: 0.0,
    },
    azimuth: 0.0,
    incline: 0.0,
    distance: 3.0,
  },

  render(imgData) {
    const start = new Date();
    addon.render(imgData, this.mainCamera);
    const end = new Date();
    console.log(`[render] time elapsed: ${end - start}`);
  },

  fillBlack(imgData) {
    const start = new Date();
    addon.fillBlack(imgData);
    const end = new Date();
    console.log(`[fillBlackRust] time elapsed: ${end - start}`);
  },
}