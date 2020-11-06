package main

import (
	"fmt"
	"os"
	"runtime"
)

func main() {
	if runtime.GOOS != "windows" {
		fmt.Fprintln(os.Stderr, "Sorry, this patch is intended to run on Windows.")
		return
	} else if len(os.Args) != 2 {
		printUsage()
		return
	}

	cssTargetFile := retrieveCssTargetFile()
	fmt.Fprintf(os.Stdout, "Patching %s ...\n", cssTargetFile)

	origContents, err := readCssContents(cssTargetFile)
	if err != nil {
		fmt.Fprintln(os.Stderr, err.Error())
		return
	}

	patchedContents, err := applyPatch(origContents)
	if err != nil {
		fmt.Fprintln(os.Stderr, err.Error())
		return
	}

	err = writeCssContents(cssTargetFile, patchedContents)
	if err != nil {
		fmt.Fprintln(os.Stderr, err.Error())
		return
	}

	fmt.Fprintln(os.Stdout, "VSCode successfully patched.")
}
