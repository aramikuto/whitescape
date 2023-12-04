import { render } from "preact"
import { useCallback, useRef, useState } from "preact/hooks"
import { gen_all } from "pkg"
import debounce from "lodash.debounce"

import "./style.css"
import { JSXInternal } from "preact/src/jsx"

const App = () => {
  const codeInputRef = useRef<HTMLTextAreaElement | null>(null)
  const [ast, setAst] = useState("")
  const [debugOutput, setDebugOutput] = useState("")
  const [whitespaceOutput, setWhitespaceOutput] = useState("")
  const codeOverlayRef = useRef<HTMLDivElement | null>(null)

  const updateAST = debounce(() => {
    const value = codeInputRef.current?.value || ""
    const output = gen_all(value)
    setAst(output.get_ast())
    setDebugOutput(output.get_debug_output())
    setWhitespaceOutput(output.get_whitespace_output())
  }, 700)

  const handleScroll = useCallback<
    JSXInternal.UIEventHandler<HTMLTextAreaElement>
  >((e) => {
    if (codeOverlayRef.current) {
      codeOverlayRef.current.scrollTop = e.currentTarget.scrollTop
      codeOverlayRef.current.scrollLeft = e.currentTarget.scrollLeft
    }
  }, [])

  const copyCode = () => {
    navigator.clipboard.writeText(whitespaceOutput)
  }

  const downloadCode = () => {
    const a = document.createElement("a")
    a.href =
      "data:text/plain;charset=utf-8," + encodeURIComponent(whitespaceOutput)
    a.download = "output.ws"
    a.click()
  }

  return (
    <div class="h-screen flex flex-col">
      <div class="flex-1 flex flex-wrap">
        <div class="w-full md:w-1/2 lg:w-1/4 h-1/2 md:h-full flex flex-col">
          <h2 class="text-center text-lg font-semibold mb-2">Source code</h2>
          <textarea
            class="w-full h-full resize-none bg-blue-200"
            ref={codeInputRef}
            onInput={updateAST}
            placeholder="Source code"
          ></textarea>
        </div>
        <div class="w-full md:w-1/2 lg:w-1/4 h-1/2 md:h-full flex flex-col">
          <h2 class="text-center text-lg font-semibold mb-2">AST</h2>
          <textarea
            class="w-full h-full resize-none bg-green-200"
            placeholder="AST"
            disabled
          >
            {ast}
          </textarea>
        </div>
        <div class="w-full md:w-1/2 lg:w-1/4 h-1/2 md:h-full flex flex-col">
          <h2 class="text-center text-lg font-semibold mb-2">Debug output</h2>
          <textarea
            class="w-full h-full resize-none bg-yellow-200"
            placeholder="Debug output"
            disabled
          >
            {debugOutput}
          </textarea>
        </div>
        <div class="w-full md:w-1/2 lg:w-1/4 h-1/2 md:h-full flex flex-col">
          <h2 class="text-center text-lg font-semibold mb-2">
            Whitespace code
          </h2>
          <div class="flex w-full h-full flex-col">
            <div class="relative h-full">
              <div
                class="w-full h-full font-xl pointer-events-none absolute whitespace-pre z-10 pb-8 overflow-scroll"
                ref={codeOverlayRef}
              >
                {Array.from(whitespaceOutput).map((c) => {
                  if (c === " ") return <span class="s">{c}</span>
                  if (c === "\n") return <span class="n">{c}</span>
                  if (c === "\t") return <span class="t">{c}</span>
                  return "?"
                })}
              </div>
              <textarea
                class="absolute inset-0 resize-none bg-red-200 font-mono text-2xl overflow-scroll"
                style={{
                  tabSize: "1rem",
                }}
                placeholder="Whitespace code"
                onScroll={handleScroll}
                disabled
              >
                {whitespaceOutput}
              </textarea>
            </div>
            <div class="self-end px-3 py-2 flex gap-4">
              <button onClick={copyCode}>Copy</button>
              <button onClick={downloadCode}>Download</button>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}

render(<App />, document.getElementById("app")!)
