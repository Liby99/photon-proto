var addon = require('../native');

module.exports = {
  render(scene, buffer) {
    for (var i = 0; i < 50; i++) {
      for (var j = 0; j < 50; j++) {
        const index = (j * 1280 + i) * 4;
        buffer[index + 0] = 0;
        buffer[index + 1] = 0;
        buffer[index + 2] = 0;
        buffer[index + 3] = 255;
      }
    }
  }
}