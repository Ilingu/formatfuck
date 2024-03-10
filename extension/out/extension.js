"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.deactivate = exports.activate = void 0;
const vscode = __importStar(require("vscode"));
const child_process_1 = __importDefault(require("child_process"));
const animate = async (animation) => {
    const halt = vscode.window.showInformationMessage("Animation is running...", "Stop");
    // lauch animation
    const activeEditor = vscode.window.activeTextEditor;
    if (activeEditor === undefined)
        return;
    const filepath = activeEditor.document.uri.fsPath;
    if (!filepath.startsWith("/home"))
        return;
    const window_view = activeEditor.visibleRanges;
    const user_selection = activeEditor.selection;
    const is_selection = user_selection.start.line !== user_selection.end.line &&
        user_selection.start.character !== user_selection.end.character;
    const window_view_string = is_selection
        ? `${user_selection.start.line}:${user_selection.end.line}`
        : `${window_view[0].start.line}:${window_view[0].end.line}`;
    const cmd = `formatfuck ${animation} ${filepath} ${window_view_string}`;
    console.log({ cmd });
    const animation_process = child_process_1.default.exec(cmd, (err, stdout, stderr) => {
        console.log({ stdout, stderr, err });
    });
    // wait user resp
    await halt;
    // stop animation
    if (!animation_process.kill())
        vscode.window.showErrorMessage("Failed to stop animation");
};
function activate(context) {
    let mir_cmd = vscode.commands.registerCommand("formatfuck_vsext.make_it_rain", () => animate("make_it_rain"));
    let gol_cmd = vscode.commands.registerCommand("formatfuck_vsext.game_of_life", () => animate("game_of_life"));
    context.subscriptions.push(mir_cmd);
    context.subscriptions.push(gol_cmd);
}
exports.activate = activate;
function deactivate() { }
exports.deactivate = deactivate;
//# sourceMappingURL=extension.js.map