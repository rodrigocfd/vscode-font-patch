package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"path/filepath"
	"runtime"
	"strings"
	"vscode-font-patch/ini"
)

func main() {
	fmt.Println("VSCode patch for font & autocomplete icon.")
	fmt.Printf("> Current system is %s.\n", runtime.GOOS)
	cssPath := getCssPath()
	cssContent := getCssContents(cssPath)
	cssChanged := false

	if runtime.GOOS == "windows" {
		if newContent, changed := patchFont(cssContent); changed {
			cssContent = newContent
			cssChanged = true
		}
	}

	if newContent, changed := patchAutocompleteIcon(cssContent); changed {
		cssContent = newContent
		cssChanged = true
	}

	if cssChanged {
		if err := ioutil.WriteFile(cssPath, []byte(cssContent), 0644); err != nil {
			panic(err)
		}
		fmt.Println("> Patch successfully applied.")
	} else {
		fmt.Println("> Nothing to do.")
	}
}

func getCssPath() string {
	const INI_FILE string = "vscode-font-patch.ini"
	const CSS string = "/resources/app/out/vs/workbench/workbench.desktop.main.css"

	settings, err := ini.Load(INI_FILE)
	if err != nil {
		panic(err)
	}

	pPath, hasPath := settings.Value("vscode", "path")
	if !hasPath {
		panic(fmt.Sprintf("VSCode path not found in %s.", INI_FILE))
	}

	cssPath := filepath.FromSlash(*pPath + CSS)
	if _, err := os.Stat(cssPath); errors.Is(err, os.ErrNotExist) {
		panic(fmt.Sprintf("CSS not found: %s", cssPath))
	}
	return cssPath
}

func getCssContents(cssPath string) string {
	rawBytes, err := ioutil.ReadFile(cssPath)
	if err != nil {
		panic(err)
	}
	return string(rawBytes)
}

func patchFont(origContent string) (string, bool) {
	const END_OF_COMMENTS string = "-*/"
	const MAGIC_PATCH string = "\n*{text-shadow:transparent 0px 0px 0px, rgba(0, 0, 0, 0.5) 0px 0px 0px !important;}"

	// Find index past the comments block.
	idxStartCode := strings.Index(origContent, END_OF_COMMENTS)
	if idxStartCode == -1 {
		panic("CSS end of comments not found.")
	}
	idxStartCode += len(END_OF_COMMENTS)

	// Is our magic path the first thing past the comments block?
	if MAGIC_PATCH == origContent[idxStartCode:idxStartCode+len(MAGIC_PATCH)] {
		fmt.Println("> Font already patched.")
		return origContent, false
	}

	return origContent[:idxStartCode] + // comments block
			MAGIC_PATCH +
			origContent[idxStartCode:], // rest of file
		true
}

func patchAutocompleteIcon(origContent string) (string, bool) {
	const NATURAL string = ".monaco-editor .suggest-widget .monaco-list .monaco-list-row.focused .codicon{color:var(--vscode-editorSuggestWidget-selectedIconForeground)}"
	const PATCHED string = " /*.monaco-editor .suggest-widget .monaco-list .monaco-list-row.focused .codicon{color:var(--vscode-editorSuggestWidget-selectedIconForeground)}*/ "

	// Is the patched string already present?
	idxPatched := strings.Index(origContent, PATCHED)
	if idxPatched != -1 {
		fmt.Println("> Autocomplete icon already patched.")
		return origContent, false
	}

	// Find index of string to be replaced.
	idxNatural := strings.Index(origContent, NATURAL)
	if idxNatural == -1 {
		panic("CSS autocomplete icon entry not found.")
	}

	return origContent[:idxNatural] + // all code up to before our string
			PATCHED +
			origContent[idxNatural+len(NATURAL):], // rest of file
		true
}
