import { chromium } from 'playwright';

const browser = await chromium.launch({
  headless: true,
  executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome'
});
const page = await browser.newPage();

const svg = `<?xml version="1.0" encoding="UTF-8"?>
<svg xmlns="http://www.w3.org/2000/svg" width="720" height="380" viewBox="0 0 720 380">
  <defs>
    <style>
      .code { font: 14px "SF Mono", Menlo, monospace; fill: #3a2a1a; }
    </style>
  </defs>
  <rect width="720" height="380" fill="#fff"/>
  <text x="80" y="115" class="code">fn add(a: i32, b: i32) -&gt; i32 { a + b }</text>
</svg>`;

await page.setContent(`<!DOCTYPE html><html><body style="margin:0;">${svg}</body></html>`);
await page.waitForTimeout(300);

const metrics = await page.evaluate(() => {
  const text = document.querySelector('text');
  const svg = document.querySelector('svg');
  const rect = text.getBBox();
  const computed = getComputedStyle(text);
  return {
    bbox: { x: rect.x, y: rect.y, width: rect.width, height: rect.height },
    fontFamily: computed.fontFamily,
    fontSize: computed.fontSize
  };
});
console.log('Text metrics:', metrics);

// Measure character widths
const charMetrics = await page.evaluate(() => {
  const text = document.querySelector('text');
  const str = text.textContent;
  const result = [];
  for (let i = 0; i < str.length; i++) {
    const rect = text.getExtentOfChar(i);
    result.push({ i, char: str[i], x: rect.x, w: rect.width });
  }
  return result;
});

// Find positions of key parts
const parts = [
  { name: 'fn', start: 0, end: 2 },
  { name: 'add', start: 3, end: 6 },
  { name: 'params', start: 7, end: 20 },
  { name: 'return-type', start: 21, end: 29 },
  { name: 'body', start: 30, end: 39 }
];

for (const part of parts) {
  const startChar = charMetrics[part.start];
  const endChar = charMetrics[part.end];
  const x1 = startChar.x;
  const x2 = endChar.x + endChar.w;
  console.log(`${part.name}: chars ${part.start}-${part.end}, x range: ${x1.toFixed(1)} - ${x2.toFixed(1)}, center: ${((x1+x2)/2).toFixed(1)}`);
}

await browser.close();
