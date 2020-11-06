package main

import (
	"fmt"
)

func printUsage() {
	fmt.Println(`
VSCode Font Aliasing

vscode-font-aliasing is a tool to slightly enhance font aliasing on VSCode.
Usually, this is necessary on Windows. It works by patching some internal
installation files.

Close VSCode before running the patch. When opening VSCode again, you will see a
warning about a corrupted installation. If you don't ignore it, the changes made
by the patch will be reverted.

Usage:

        vscode-font-aliasing <go-install-folder>

Example:

        vscode-font-aliasing "C:\My stuff\Microsoft VS Code"
		`)
}
