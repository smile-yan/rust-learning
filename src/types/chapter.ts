export type Exercise = {
  title: string;
  description: string;
  code_template: string;
};

export type Chapter = {
  title: string;
  hint?: string;
  code: string;
  exercises: Exercise[];
  htmlPath: string;
};

export type Module = {
  name: string;
  chapters: Chapter[];
};

export type Manifest = {
  modules: Module[];
};

export type ChapterLocation = {
  moduleIdx: number;
  chapterIdxInModule: number;
};
