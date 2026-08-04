import { ref } from "vue";

const DEFAULT_EVALUATE_URL = "http://localhost:9001/evaluate.json";

type PlaygroundResponse = {
  success?: boolean;
  stdout?: string;
  stderr?: string;
  error?: string | null;
  result?: string;
};

export function usePlayground() {
  const outputText = ref("点击「运行」按钮查看输出结果");
  const outputClass = ref("text-gray-400");
  const elapsedTime = ref<number | null>(null);

  function clearOutput() {
    outputText.value = "点击「运行」按钮查看输出结果";
    outputClass.value = "text-gray-400";
    elapsedTime.value = null;
  }

  function renderOutput(data: PlaygroundResponse) {
    const stdout = typeof data.stdout === "string" ? data.stdout : "";
    const stderr = typeof data.stderr === "string" ? data.stderr : "";
    const result = typeof data.result === "string" ? data.result : "";
    const error = data.error !== null && data.error !== undefined ? String(data.error) : "";

    const output = stdout || result;

    if (error.length > 0) {
      outputText.value = error;
      outputClass.value = "text-red-400";
    } else if (stderr.length > 0 && output.length === 0) {
      outputText.value = stderr;
      outputClass.value = "text-orange-400";
    } else if (output.length > 0) {
      outputText.value = stderr.length > 0 ? `${output}\n${stderr}` : output;
      outputClass.value = "text-green-400";
    } else {
      outputText.value = "（程序没有输出）";
      outputClass.value = "text-gray-200";
    }
  }

  async function runCode(code: string) {
    outputText.value = "⏳ 正在编译运行，请稍候...";
    outputClass.value = "text-gray-200";
    const startTime = performance.now();

    const evaluateUrl = import.meta.env.VITE_EVALUATE_URL || DEFAULT_EVALUATE_URL;

    try {
      const controller = new AbortController();
      const timeoutId = setTimeout(() => controller.abort(), 25000);

      const res = await fetch(evaluateUrl, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          version: "stable",
          edition: "2021",
          crateType: "bin",
          mode: "debug",
          tests: false,
          optimize: "0",
          code,
        }),
        signal: controller.signal,
      });

      clearTimeout(timeoutId);

      if (!res.ok) {
        throw new Error(`服务器返回 HTTP ${res.status}`);
      }

      const data = (await res.json()) as PlaygroundResponse;
      renderOutput(data);
    } catch (err) {
      if (err instanceof Error && err.name === "AbortError") {
        outputText.value = "❌ 请求超时，请检查网络或稍后重试。";
      } else if (err instanceof Error) {
        outputText.value = `❌ 运行失败: ${err.message}`;
      } else {
        outputText.value = "❌ 运行失败";
      }
      outputClass.value = "text-red-400";
    } finally {
      elapsedTime.value = (performance.now() - startTime) / 1000;
    }
  }

  return { outputText, outputClass, elapsedTime, runCode, clearOutput };
}
