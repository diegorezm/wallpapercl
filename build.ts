import {build, file, spawn} from "bun";

async function buildApp() {
  console.log("Building React/TypeScript application...");
  await build({
    entrypoints: ["internal/web/_client.tsx"],
    outdir: "dist",
    minify: true,
    target: "browser",
  });

  console.log("Copying index.html...");
  const input = file("./public/index.html");
  const output = file("./dist/index.html");
  await Bun.write(output, input);

  console.log("Building Tailwind CSS...");
  spawn([
    "bunx",
    "tailwindcss",
    "-i",
    "./public/styles.css",
    "-o",
    "./dist/styles.css",
    "--minify",
  ]);
  console.log("Build completed!");
}

await buildApp();

