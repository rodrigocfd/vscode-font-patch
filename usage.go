package main

import (
	"fmt"
)

func printUsage() {
	fmt.Println(`
VSCode Font Patch

vscode-font-patch is a tool to slightly enhance font rendering on VSCode.
Usually, this is necessary on Windows. It works by patching some internal
installation files.

Close VSCode before running the patch. When opening VSCode again, you will see a
warning about a corrupted installation. If you don't ignore it, the changes made
by the patch will be reverted.

Usage:

        vscode-font-patch <go-install-folder>

Example:

        vscode-font-patch "C:\My stuff\Microsoft VS Code"
		`)
}
