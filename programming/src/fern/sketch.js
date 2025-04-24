function setup() {
  const baseW = 1024;
  const baseH = 1024;
  const scaling = Math.min(windowWidth / baseW, windowHeight / baseH);
  const canvas = createCanvas(baseW, baseH);
  canvas.elt.style.width = `${baseW * scaling}px`;
  canvas.elt.style.height = `${baseH * scaling}px`;
  background(0);
  strokeWeight(1.1);
  stroke(120, 255, 110, 100);

  const w1x = (x, y) => 0.836 * x + 0.044 * y;
  const x1y = (x, y) => -0.044 * x + 0.836 * y + 0.169;
  const w2x = (x, y) => -0.141 * x + 0.302 * y;
  const w2y = (x, y) => 0.302 * x + 0.141 * y + 0.127;
  const w3x = (x, y) => 0.141 * x - 0.302 * y;
  const w3y = (x, y) => 0.302 * x + 0.141 * y + 0.169;
  const w4x = (_x, _y) => 0.0;
  const w4y = (_x, y) => 0.175337 * y;
  for (const st = [[23, 512, 512]]; st.length > 0;) {
    const [k, x, y] = st.pop();
    if (k > 0) {
      st.push([k - 1, w1x(x, y), x1y(x, y)]);
      if (Math.random() < 0.3) {
        st.push([k - 1, w2x(x, y), w2y(x, y)]);
      }
      if (Math.random() < 0.3) {
        st.push([k - 1, w3x(x, y), w3y(x, y)]);
      }
      if (Math.random() < 0.3) {
        st.push([k - 1, w4x(x, y), w4y(x, y)]);
      }
    }
    const ss = 1024 * 0.97;
    const xx = Math.floor(x * ss + 1024 * 0.5) - 1;
    const yy = Math.floor(1024 - y * ss) - 1;
    point(xx, yy);
  }
}
