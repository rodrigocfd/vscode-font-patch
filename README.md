# VSCode Font Patch

A command line utility to patch a Visual Studio Code installation, slightly enhancing the font rendering on Windows.

Also fixes the autocomplete highlighted icon for both Windows and Linux, which is messed up [since February 2021 release](https://stackoverflow.com/q/68321114/6923555).

## Comparison

* Using Consolas font (original, patched)

![Diff Consolas](diff-consolas.png)

* Using Source Code Pro font (original, patched)

![Diff Source Code Pro](diff-sourcecodepro.png)

## Usage

1. Write the proper VSCode installation path in `vscode-font-patch.ini`;
2. Close VSCode, if running;
3. Run the patch.

Notice that, if you installed VSCode in Linux with `sudo`, you'll need to run the patch with `sudo` as well.

When opening VSCode again, you will see a warning about a corrupted installation. If you don't ignore it, the changes made by the patch will be reverted.

To revert the patch, simply run VS Code installer again.

## Build

To build the program, run:

    go build -ldflags "-s -w"
