import fs from "node:fs/promises";
import path from "node:path";
import matter from "gray-matter";
import { marked } from "marked";
import yaml from "js-yaml";

const ROOT = path.resolve(import.meta.dirname, "..");
const CONTENT_DIR = path.join(ROOT, "content", "modules");
const OUTPUT_DIR = path.join(ROOT, "public", "chapters");

type Exercise = {
  title: string;
  description: string;
  code_template: string;
};

type ChapterMeta = {
  title: string;
  hint?: string;
};

type ChapterOutput = ChapterMeta & {
  code: string;
  exercises: Exercise[];
  htmlPath: string;
};

type ModuleOutput = {
  name: string;
  chapters: ChapterOutput[];
};

type Manifest = {
  modules: ModuleOutput[];
};

async function readDirNames(dir: string): Promise<string[]> {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  return entries
    .filter((e) => e.isDirectory())
    .map((e) => e.name)
    .sort();
}

async function main() {
  await fs.mkdir(OUTPUT_DIR, { recursive: true });

  const moduleDirs = await readDirNames(CONTENT_DIR);
  const manifest: Manifest = { modules: [] };

  for (let mi = 0; mi < moduleDirs.length; mi++) {
    const modDir = path.join(CONTENT_DIR, moduleDirs[mi]);
    const meta = yaml.load(await fs.readFile(path.join(modDir, "meta.yaml"), "utf-8")) as {
      name: string;
    };

    const chaptersDir = path.join(modDir, "chapters");
    const chapterDirs = await readDirNames(chaptersDir);

    const moduleOutput: ModuleOutput = { name: meta.name, chapters: [] };

    for (let ci = 0; ci < chapterDirs.length; ci++) {
      const chDir = path.join(chaptersDir, chapterDirs[ci]);

      const mdRaw = await fs.readFile(path.join(chDir, "chapter.md"), "utf-8");
      const parsed = matter(mdRaw);
      const frontmatter = parsed.data as ChapterMeta;

      // Strip cache-busting query strings from image references so they
      // resolve against the static public/images directory.
      const cleanMd = parsed.content.replace(
        /(!\[.*?\]\(images\/[^?)]+)(\?[^)]*)?(\))/g,
        "$1$3"
      );

      const html = await marked.parse(cleanMd);

      const code = await fs.readFile(path.join(chDir, "main.rs"), "utf-8");

      const exercisesRaw = await fs.readFile(path.join(chDir, "exercises.yaml"), "utf-8");
      const exercises = ((yaml.load(exercisesRaw) as Exercise[] | null) ?? []).filter(
        (e): e is Exercise => !!e
      );

      const htmlDir = path.join(OUTPUT_DIR, String(mi), String(ci));
      await fs.mkdir(htmlDir, { recursive: true });
      const htmlRelPath = path.join(String(mi), String(ci), "index.html");
      await fs.writeFile(path.join(OUTPUT_DIR, htmlRelPath), html, "utf-8");

      moduleOutput.chapters.push({
        title: frontmatter.title,
        hint: frontmatter.hint,
        code,
        exercises,
        htmlPath: `chapters/${htmlRelPath.replace(/\\/g, "/")}`
      });
    }

    manifest.modules.push(moduleOutput);
  }

  await fs.writeFile(
    path.join(OUTPUT_DIR, "manifest.json"),
    JSON.stringify(manifest, null, 2),
    "utf-8"
  );

  const totalChapters = manifest.modules.reduce((sum, m) => sum + m.chapters.length, 0);
  console.log(`Built content: ${manifest.modules.length} modules, ${totalChapters} chapters`);
}

main().catch((err) => {
  console.error(err);
  process.exit(1);
});
