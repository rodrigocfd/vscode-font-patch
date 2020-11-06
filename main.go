package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"os"
	"runtime"
	"strings"
)

func main() {
	if runtime.GOOS != "windows" {
		fmt.Fprintln(os.Stderr, "Sorry, this patch is intended to run on Windows.")

	} else if len(os.Args) != 2 {
		printUsage()

	} else {
		fmt.Fprintln(os.Stdout, "Patching...")

		cssTargetFile := retrieveCssTargetFile()
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
}

func retrieveCssTargetFile() string {
	cssTarget := os.Args[1]
	if !strings.HasSuffix(cssTarget, "\\") {
		cssTarget += "\\"
	}
	return cssTarget + "resources\\app\\out\\vs\\workbench\\workbench.desktop.main.css"
}

func readCssContents(cssTargetFile string) (string, error) {
	fin, err := os.OpenFile(cssTargetFile, os.O_RDONLY, 0)
	if err != nil {
		return "", err
	}
	defer fin.Close()

	byteContents, err := ioutil.ReadAll(fin)
	if err != nil {
		return "", err
	}

	fmt.Fprintf(os.Stdout, "Read: %d bytes.\n", len(byteContents))
	return string(byteContents), nil
}

func applyPatch(contents string) (string, error) {
	endOfComments := "-*/"
	idxStartCode := strings.Index(contents, endOfComments) +
		len(endOfComments)

	const magicPatch = "*{text-shadow:transparent 0px 0px 0px, rgba(0, 0, 0, 0.5) 0px 0px 0px !important;}"

	if magicPatch == contents[idxStartCode:idxStartCode+len(magicPatch)] {
		return "", errors.New("Aborted: this instalation is already patched.")
	}

	patched := strings.Builder{}
	patched.WriteString(contents[:idxStartCode])
	patched.WriteString(magicPatch)
	patched.WriteString(contents[idxStartCode:])

	return patched.String(), nil
}

func writeCssContents(cssTargetFile, contents string) error {
	fout, err := os.OpenFile(cssTargetFile, os.O_RDWR|os.O_TRUNC, os.ModeExclusive)
	if err != nil {
		return err
	}
	defer fout.Close()

	bytesWritten, err := fout.WriteString(contents)
	if err != nil {
		return err
	}

	fmt.Fprintf(os.Stdout, "Written: %d bytes.\n", bytesWritten)
	return nil
}
