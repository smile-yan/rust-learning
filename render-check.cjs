const { chromium } = require("playwright");
(async () => {
  const files = process.argv.slice(2);
  const browser = await chromium.launch();
  for (const f of files) {
    const page = await browser.newPage({ viewport: { width: 760, height: 520 } });
    await page.goto("file://" + process.cwd() + "/" + f);
    await page.waitForTimeout(300);
    const out = f.replace("images/", "").replace(".svg", "") + ".png";
    await page.screenshot({ path: out });
    console.log("rendered", out);
    await page.close();
  }
  await browser.close();
})();
