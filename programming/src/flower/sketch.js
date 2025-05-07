let hand, mouth;

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
  const img = drawShiftGlitch(drawColorGlitch(drawImage(), 5), 10);
  image(img, 0, 0);
  stroke(0, 50);
  strokeWeight(1);
  for (let i = 0; i < height; i += height / 200) {
    line(0, i, width, i);
  }
}

function drawImage() {
  push();

  imageMode(CENTER);
  translate(width / 2, height / 2);

  const petals = floor(random(10, 20));
  const radius = min(width, height) * 0.35;

  for (let i = 0; i < petals; i++) {
    push();
    const angle = TWO_PI / petals * i + random(-0.05, 0.05);
    rotate(angle);
    const s = random(0.6, 1.2);
    scale(s);
    {
      push();
      translate(0, -radius);
      rotate(-PI / 2);
      tint(
        random(100, 255),
        random(100, 255),
        random(100, 255),
        random(150, 255)
      );
      image(hand, 0, 0);
      pop();
    }
    pop();
  }
  {
    push();
    noTint();

    const mouthSize = radius * random(0.4, 0.6);
    image(mouth, 0, 0, mouthSize * 2.478, mouthSize);
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
