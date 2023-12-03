import { render } from "preact"
import { useRef, useState } from "preact/hooks"
import { gen_all } from "pkg"
import debounce from "lodash.debounce"

import "./style.css"

const App = () => {
  const codeInputRef = useRef<HTMLTextAreaElement | null>(null)
  const [ast, setAst] = useState("")
  const [debugOutput, setDebugOutput] = useState("")
  const [whitespaceOutput, setWhitespaceOutput] = useState("")

  const updateAST = debounce(() => {
    const value = codeInputRef.current?.value || ""
    const output = gen_all(value)
    setAst(output.get_ast())
    setDebugOutput(output.get_debug_output())
    setWhitespaceOutput(output.get_whitespace_output())
  }, 700)

  return (
    <div class="h-screen flex flex-col">
      <div class="flex-1 flex flex-wrap">
        <div class="w-full md:w-1/2 lg:w-1/4 h-1/2 md:h-full flex flex-col">
          <h2 class="text-center text-lg font-semibold mb-2">Source code</h2>
          <textarea
            class="w-full h-full textarea bg-blue-200"
            ref={codeInputRef}
            onInput={updateAST}
            placeholder="Source code"
          ></textarea>
        </div>
        <div class="w-full md:w-1/2 lg:w-1/4 h-1/2 md:h-full flex flex-col">
          <h2 class="text-center text-lg font-semibold mb-2">AST</h2>
          <textarea
            class="w-full h-full textarea bg-green-200"
            placeholder="AST"
            disabled
          >
            {ast}
          </textarea>
        </div>
        <div class="w-full md:w-1/2 lg:w-1/4 h-1/2 md:h-full flex flex-col">
          <h2 class="text-center text-lg font-semibold mb-2">Debug output</h2>
          <textarea
            class="w-full h-full textarea bg-yellow-200"
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
          <textarea
            class="w-full h-full textarea bg-red-200"
            placeholder="Whitespace code"
            disabled
          >
            {whitespaceOutput}
          </textarea>
        </div>
      </div>
    </div>
  )
}

render(<App />, document.getElementById("app")!)
