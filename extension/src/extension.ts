import * as vscode from "vscode";
import cp from "child_process";

const animate = async (animation: "make_it_rain" | "game_of_life") => {
  const halt = vscode.window.showInformationMessage(
    "Animation is running...",
    "Stop"
  );

  // lauch animation
  const activeEditor = vscode.window.activeTextEditor;
  if (activeEditor === undefined) return;

  const filepath = activeEditor.document.uri.fsPath;
  if (!filepath.startsWith("/home")) return;
  const window_view = activeEditor.visibleRanges;
  const user_selection = activeEditor.selection;

  const is_selection =
    user_selection.start.line !== user_selection.end.line &&
    user_selection.start.character !== user_selection.end.character;

  const window_view_string = is_selection
    ? `${user_selection.start.line}:${user_selection.end.line}`
    : `${window_view[0].start.line}:${window_view[0].end.line}`;

  const cmd = `formatfuck ${animation} ${filepath} ${window_view_string}`;
  console.log({ cmd });

  const animation_process = cp.exec(cmd, (err, stdout, stderr) => {
    console.log({ stdout, stderr, err });
  });

  // wait user resp
  await halt;
  // stop animation
  if (!animation_process.kill())
    vscode.window.showErrorMessage("Failed to stop animation");
};

export function activate(context: vscode.ExtensionContext) {
  let mir_cmd = vscode.commands.registerCommand(
    "formatfuck_vsext.make_it_rain",
    () => animate("make_it_rain")
  );
  let gol_cmd = vscode.commands.registerCommand(
    "formatfuck_vsext.game_of_life",
    () => animate("game_of_life")
  );

  context.subscriptions.push(mir_cmd);
  context.subscriptions.push(gol_cmd);
}

export function deactivate() {}
