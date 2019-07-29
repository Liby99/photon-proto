var addon = require('../native');

module.exports = {
  render(imgData) {
    const start = new Date();
    addon.render(imgData);
    const end = new Date();
    console.log(`[renderRust] time elapsed: ${end - start}`);
  },

  fillBlack(imgData) {
    const start = new Date();
    addon.fillBlack(imgData);
    const end = new Date();
    console.log(`[fillBlackRust] time elapsed: ${end - start}`);
  },

  fillBlackJs(imgData) {
    const start = new Date();
    for (var i = 0; i < imgData.width; i++) {
      for (var j = 0; j < imgData.height; j++) {
        const index = (j * imgData.width + i) * 4;
        imgData.data[index] = 0;
        imgData.data[index + 1] = 0;
        imgData.data[index + 2] = 0;
        imgData.data[index + 3] = 255;
      }
    }
    const end = new Date();
    console.log(`[fillBlackJs] time elapsed: ${end - start}`);
  }
}