import fs from "node:fs/promises";
import path from "node:path";
import yaml from "js-yaml";

const ROOT = path.resolve(import.meta.dirname, "..");
const SOURCE = path.join(ROOT, "js", "chapters.json");
const DEST = path.join(ROOT, "content", "modules");

type Exercise = {
  title: string;
  description: string;
  code_template: string;
};

type Chapter = {
  title: string;
  theory: string;
  code: string;
  hint?: string;
  exercises?: Exercise[];
};

type Module = {
  name: string;
  chapters: Chapter[];
};

function slugify(title: string, fallback: string): string {
  const matches = title.match(/[a-zA-Z0-9!_/<>'"]+/g);
  if (!matches || matches.length === 0) {
    return fallback;
  }
  const raw = matches.join("-");
  const slug = raw
    .toLowerCase()
    .replace(/[^a-z0-9!]+/g, "-")
    .replace(/^-+|-+$/g, "")
    .replace(/-+/g, "-");
  return slug || fallback;
}

async function main() {
  const data: Module[] = JSON.parse(await fs.readFile(SOURCE, "utf-8"));
  await fs.mkdir(DEST, { recursive: true });

  for (let mi = 0; mi < data.length; mi++) {
    const mod = data[mi];
    const modDir = path.join(DEST, `${String(mi).padStart(2, "0")}`);
    const chaptersDir = path.join(modDir, "chapters");
    await fs.mkdir(chaptersDir, { recursive: true });
    await fs.writeFile(path.join(modDir, "meta.yaml"), `name: ${mod.name}\n`, "utf-8");

    for (let ci = 0; ci < mod.chapters.length; ci++) {
      const ch = mod.chapters[ci];
      const chDir = path.join(chaptersDir, `${String(ci + 1).padStart(2, "0")}`);
      await fs.mkdir(chDir, { recursive: true });

      const frontmatter: Record<string, unknown> = { title: ch.title };
      if (ch.hint) frontmatter.hint = ch.hint;

      const fmYaml = yaml.dump(frontmatter, { lineWidth: -1 }).trim();
      const chapterMd = `---\n${fmYaml}\n---\n\n${ch.theory.trimStart()}\n`;
      await fs.writeFile(path.join(chDir, "chapter.md"), chapterMd, "utf-8");

      await fs.writeFile(path.join(chDir, "main.rs"), ch.code, "utf-8");

      const exercises = ch.exercises ?? [];
      await fs.writeFile(
        path.join(chDir, "exercises.yaml"),
        yaml.dump(exercises, { lineWidth: -1 }),
        "utf-8"
      );
    }
  }

  console.log(`Migrated ${data.length} modules to ${DEST}`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
