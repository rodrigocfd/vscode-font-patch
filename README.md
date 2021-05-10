# VSCode Font Patch

A command line utility to patch a Visual Studio Code installation on **Windows**, slightly enhancing the font rendering.

## Usage

Close VSCode and run the application.

![Screenshot](screenshot.png)

When opening VSCode again, you will see a warning about a corrupted installation. If you don't ignore it, the changes made by the patch will be reverted.

To revert the patch, simply run VS Code installer again.

## Comparison before/after patch

* Using Consolas font (original vs. patched)

![Diff Consolas](diff-consolas.png)

* Using Source Code Pro font (original vs. patched)

![Diff Source Code Pro](diff-sourcecodepro.png)

## License

Licensed under [MIT license](https://opensource.org/licenses/MIT), see [LICENSE.md](LICENSE.md) for details.
