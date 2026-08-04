import fs from "node:fs/promises";
import path from "node:path";
import matter from "gray-matter";
import yaml from "js-yaml";

const ROOT = path.resolve(import.meta.dirname, "..");
const CONTENT_DIR = path.join(ROOT, "content", "modules");
const IMAGES_DIR = path.join(ROOT, "public", "images");

let errors = 0;
let warnings = 0;

function error(msg: string) {
  errors++;
  console.error(`❌ ${msg}`);
}

function warn(msg: string) {
  warnings++;
  console.warn(`⚠️  ${msg}`);
}

async function fileExists(p: string): Promise<boolean> {
  try {
    await fs.access(p);
    return true;
  } catch {
    return false;
  }
}

async function readDirNames(dir: string): Promise<string[]> {
  const entries = await fs.readdir(dir, { withFileTypes: true });
  return entries
    .filter((e) => e.isDirectory())
    .map((e) => e.name)
    .sort();
}

async function validate() {
  if (!(await fileExists(CONTENT_DIR))) {
    error(`content/modules/ directory not found at ${CONTENT_DIR}`);
    return;
  }

  const moduleDirs = await readDirNames(CONTENT_DIR);
  if (moduleDirs.length === 0) {
    error("No modules found in content/modules/");
    return;
  }

  let totalChapters = 0;
  const imageRefs = new Set<string>();

  for (const modDirName of moduleDirs) {
    const modDir = path.join(CONTENT_DIR, modDirName);
    const metaPath = path.join(modDir, "meta.yaml");

    if (!(await fileExists(metaPath))) {
      error(`Module ${modDirName}: missing meta.yaml`);
      continue;
    }

    const meta = yaml.load(await fs.readFile(metaPath, "utf-8")) as Record<string, unknown>;
    if (!meta.name || typeof meta.name !== "string") {
      error(`Module ${modDirName}: meta.yaml must have a string "name"`);
    }

    const chaptersDir = path.join(modDir, "chapters");
    if (!(await fileExists(chaptersDir))) {
      error(`Module ${modDirName}: missing chapters/ directory`);
      continue;
    }

    const chapterDirs = await readDirNames(chaptersDir);
    if (chapterDirs.length === 0) {
      warn(`Module ${modDirName}: no chapters found`);
    }

    for (const chDirName of chapterDirs) {
      const chDir = path.join(chaptersDir, chDirName);
      const chapterMdPath = path.join(chDir, "chapter.md");
      const mainRsPath = path.join(chDir, "main.rs");
      const exercisesPath = path.join(chDir, "exercises.yaml");

      if (!(await fileExists(chapterMdPath))) {
        error(`Module ${modDirName}/${chDirName}: missing chapter.md`);
        continue;
      }
      if (!(await fileExists(mainRsPath))) {
        error(`Module ${modDirName}/${chDirName}: missing main.rs`);
      }
      if (!(await fileExists(exercisesPath))) {
        error(`Module ${modDirName}/${chDirName}: missing exercises.yaml`);
        continue;
      }

      const mdRaw = await fs.readFile(chapterMdPath, "utf-8");
      const parsed = matter(mdRaw);
      const title = parsed.data.title;
      if (!title || typeof title !== "string") {
        error(`Module ${modDirName}/${chDirName}: chapter.md frontmatter missing "title"`);
      }

      const theory = parsed.content;
      if (theory.trim().length < 50) {
        warn(`Module ${modDirName}/${chDirName}: theory content is very short`);
      }

      const refs = theory.match(/!\[.*?\]\((images\/[^)]+)\)/g) ?? [];
      for (const ref of refs) {
        const m = ref.match(/!\[.*?\]\((images\/[^)]+)\)/);
        if (m) imageRefs.add(m[1]);
      }

      const exercisesRaw = await fs.readFile(exercisesPath, "utf-8");
      const exercises = (yaml.load(exercisesRaw) as unknown[] | null) ?? [];
      if (!Array.isArray(exercises)) {
        error(`Module ${modDirName}/${chDirName}: exercises.yaml must be an array`);
        continue;
      }

      for (let i = 0; i < exercises.length; i++) {
        const ex = exercises[i] as Record<string, unknown>;
        if (!ex.title || typeof ex.title !== "string") {
          error(`Module ${modDirName}/${chDirName}/exercise ${i + 1}: missing "title"`);
        }
        if (!ex.description || typeof ex.description !== "string") {
          error(`Module ${modDirName}/${chDirName}/exercise ${i + 1}: missing "description"`);
        }
        if (!ex.code_template || typeof ex.code_template !== "string") {
          error(`Module ${modDirName}/${chDirName}/exercise ${i + 1}: missing "code_template"`);
        }
      }

      totalChapters++;
    }
  }

  for (const ref of imageRefs) {
    const cleanRef = ref.split("?")[0];
    const imgPath = path.join(IMAGES_DIR, path.basename(cleanRef));
    if (!(await fileExists(imgPath))) {
      warn(`Referenced image missing: ${ref} (looked at ${imgPath})`);
    }
  }

  console.log(`\nModules: ${moduleDirs.length}, Chapters: ${totalChapters}`);
  console.log(`Image refs checked: ${imageRefs.size}`);

  if (errors > 0 || warnings > 0) {
    console.log(`\nErrors: ${errors}, Warnings: ${warnings}`);
  }

  if (errors > 0) {
    process.exit(1);
  }

  console.log("✅ Content validation passed");
}

validate().catch((err) => {
  console.error(err);
  process.exit(1);
});
