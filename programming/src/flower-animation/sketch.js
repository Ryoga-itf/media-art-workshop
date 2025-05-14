let hand, mouth;
let petals;
let baseR;

function preload() {
  hand = loadImage("hand.png");
  mouth = loadImage("mouth.png");
}

function setup() {
  const baseW = 728;
  const baseH = 500;
  const scaling = Math.min(windowWidth / baseW, windowHeight / baseH);
  const canvas = createCanvas(baseW, baseH);
  canvas.elt.style.width = `${baseW * scaling}px`;
  canvas.elt.style.height = `${baseH * scaling}px`;

  background(0);

  petals = floor(random(10, 20));
  baseR = min(width, height) * 0.35;
}

function draw() {
  background(0, 30);

  const img = drawShiftGlitch(drawColorGlitch(drawFlower(), 5), 10);
  image(img, 0, 0);

  stroke(0, 50);
  strokeWeight(1);
  for (let i = 0; i < height; i += height / 200) {
    line(0, i, width, i);
  }

}

function drawFlower() {
  push();

  colorMode(HSB, 360, 100, 100, 100);
  imageMode(CENTER);
  translate(width / 2, height / 2);
  rotate(frameCount * 0.003);

  for (let i = 0; i < petals; i++) {
    push();
    const angle = TWO_PI / petals * i;
    rotate(angle);

    const dynamicR = baseR + sin(frameCount * 0.02 + i) * 15;
    translate(0, -dynamicR);

    rotate(-PI / 2);

    const s = map(sin(frameCount * 0.015 + i), -1, 1, 0.8, 1.2);
    scale(s);

    tint((frameCount * 0.5 + i * (360 / petals)) % 360, 80, 70, 80);

    image(hand, 0, 0);
    pop();
  }

  {
    push();
    noTint();
    const mouthSize = baseR * 0.6;
    image(mouth, 0, 0, mouthSize, mouthSize);
    pop();
  }

  pop();

  const img = get();
  clear();
  return img;
}

function drawColorGlitch(img, shift_size) {
  push();
  blendMode(ADD);
  tint(color(255, 0, 0));
  image(img, -shift_size, 0);
  tint(color(0, 255, 255));
  image(img, shift_size, 0);
  pop();

  const img_glitch = get();
  clear();
  return img_glitch;
}

function drawShiftGlitch(img, shift_size) {
  image(img, 0, 0);

  for (let i = 0; i < 100; i++) {
    const sx = random(img.width * 0.5);
    const sy = random(img.height * 0.05);
    const x = random(img.width - sx * 0.5);
    const y = random(img.height - sy * 0.5);
    const ix = x + random(-1, 1) * shift_size;
    const iy = y;
    image(img, ix, iy, sx, sy, x, y, sx, sy);
  }

  const img_glitch = get();
  clear();
  return img_glitch;
}
