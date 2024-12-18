import {build, spawn} from "bun";

async function buildApp() {
  // Bundle JavaScript using Bun
  console.log("Bundling typescript...");
  build({
    entrypoints: ["./public/index.js"],
    outdir: "./dist",
    minify: true,
    target: "browser",
  }).then(() => {
    console.log("Typescript bundled!")
  }).catch(e => {
    console.error(e)
  })

  console.log("Bundling tailwind...")

  spawn([
    "bunx",
    "tailwindcss",
    "-i",
    "./public/styles.css",
    "-o",
    "./dist/styles.css",
    "--minify",
  ]);
}

await buildApp();
