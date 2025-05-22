import * as vscode from 'vscode';
import * as cp from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

// Collection that holds the diagnostics for our files
let diagnosticCollection: vscode.DiagnosticCollection;

// Severity mapping from analyzer to VSCode
const severityMap: { [key: string]: vscode.DiagnosticSeverity } = {
  'error': vscode.DiagnosticSeverity.Error,
  'warning': vscode.DiagnosticSeverity.Warning,
  'info': vscode.DiagnosticSeverity.Information
};

export function activate(context: vscode.ExtensionContext) {
  console.log('Orbit UI Tools extension is now active');

  // Create diagnostic collection
  diagnosticCollection = vscode.languages.createDiagnosticCollection('orbit');
  context.subscriptions.push(diagnosticCollection);

  // Register commands
  context.subscriptions.push(
    vscode.commands.registerCommand('orbit.analyzer.analyzeCurrentFile', analyzeCurrentFile),
    vscode.commands.registerCommand('orbit.analyzer.analyzeWorkspace', analyzeWorkspace)
  );

  // Register event handlers
  context.subscriptions.push(
    vscode.workspace.onDidSaveTextDocument((document: vscode.TextDocument) => {
      if (shouldValidateDocument(document)) {
        analyzeDocument(document);
      }
    }),
    vscode.workspace.onDidOpenTextDocument((document: vscode.TextDocument) => {
      if (shouldValidateDocument(document)) {
        analyzeDocument(document);
      }
    }),
    vscode.workspace.onDidChangeTextDocument((event: vscode.TextDocumentChangeEvent) => {
      if (shouldValidateOnType() && shouldValidateDocument(event.document)) {
        analyzeDocument(event.document);
      }
    })
  );

  // Initial analysis of open documents
  vscode.workspace.textDocuments.forEach((document: vscode.TextDocument) => {
    if (shouldValidateDocument(document)) {
      analyzeDocument(document);
    }
  });
}

// Determine if we should analyze this document
function shouldValidateDocument(document: vscode.TextDocument): boolean {
  const config = vscode.workspace.getConfiguration('orbit.analyzer');
  return config.get<boolean>('enable', true) && 
         document.languageId === 'orbit' &&
         document.uri.scheme === 'file';
}

// Determine if we should validate on type
function shouldValidateOnType(): boolean {
  const config = vscode.workspace.getConfiguration('orbit.analyzer');
  return config.get<boolean>('validateOnType', false);
}

// Get the analyzer executable path
function getAnalyzerPath(): string {
  const config = vscode.workspace.getConfiguration('orbit.analyzer');
  return config.get<string>('path', 'orbit-analyzer');
}

// Analyze the current document
async function analyzeCurrentFile() {
  const editor = vscode.window.activeTextEditor;
  if (editor && editor.document.languageId === 'orbit') {
    await analyzeDocument(editor.document);
    vscode.window.showInformationMessage('Orbit file analyzed');
  } else {
    vscode.window.showErrorMessage('No Orbit file is currently open');
  }
}

// Analyze all orbit files in the workspace
async function analyzeWorkspace() {
  const workspaceFolders = vscode.workspace.workspaceFolders;
  if (!workspaceFolders) {
    vscode.window.showErrorMessage('No workspace folder is open');
    return;
  }

  // Show progress indicator
  await vscode.window.withProgress({
    location: vscode.ProgressLocation.Notification,
    title: 'Analyzing Orbit files...',
    cancellable: true
  }, async (progress: vscode.Progress<{ increment?: number; message?: string }>, token: vscode.CancellationToken) => {
    progress.report({ increment: 0 });

    // Find all .orbit files in the workspace
    const pattern = new vscode.RelativePattern(workspaceFolders[0], '**/*.orbit');
    const files = await vscode.workspace.findFiles(pattern);
    
    // Analyze each file
    let filesProcessed = 0;
    for (const file of files) {
      if (token.isCancellationRequested) {
        break;
      }
      
      const document = await vscode.workspace.openTextDocument(file);
      await analyzeDocument(document);
      
      filesProcessed++;
      progress.report({ increment: (100 / files.length), message: `${filesProcessed}/${files.length} files processed` });
    }
  });

  vscode.window.showInformationMessage('Workspace analysis complete');
}

// Analyze a single document
async function analyzeDocument(document: vscode.TextDocument) {
  // Skip if not an orbit file
  if (document.languageId !== 'orbit' || document.uri.scheme !== 'file') {
    return;
  }
  
  // Clear previous diagnostics
  diagnosticCollection.delete(document.uri);
  
  // Get configuration
  const config = vscode.workspace.getConfiguration('orbit.analyzer');
  const analyzerPath = getAnalyzerPath();
  const rules = config.get<string[]>('rules', []);
  const configPath = config.get<string>('configPath', '');
  
  // Build the command arguments
  const args = ['analyze', '--format', 'json'];
  
  // Add config path if specified
  if (configPath) {
    args.push('--config', configPath);
  }
  
  // Add rules if specified
  if (rules.length > 0) {
    args.push('--rules', rules.join(','));
  }
  
  // Add the file path
  args.push(document.uri.fsPath);
  
  try {
    // Execute the analyzer
    const output = await executeCommand(analyzerPath, args);
    const issues = JSON.parse(output);
    
    // Convert issues to diagnostics
    const diagnostics: vscode.Diagnostic[] = [];
    for (const issue of issues) {
      const startLine = Math.max(0, issue.line - 1);
      const startColumn = Math.max(0, issue.column - 1);
      
      // Get the affected line from the document
      const line = document.lineAt(startLine);
      
      // Create a range that covers the issue
      const range = new vscode.Range(
        startLine, startColumn,
        startLine, line.text.length
      );
      
      // Create a diagnostic
      const severity = severityMap[issue.severity.toLowerCase()] || vscode.DiagnosticSeverity.Warning;
      const diagnostic = new vscode.Diagnostic(
        range,
        issue.message,
        severity
      );
      
      diagnostic.code = issue.rule;
      diagnostic.source = 'orbit-analyzer';
      
      diagnostics.push(diagnostic);
    }
    
    // Update diagnostics
    diagnosticCollection.set(document.uri, diagnostics);
  } catch (error) {
    console.error('Error running orbit-analyzer:', error);
  }
}

// Execute a command and return its output
function executeCommand(command: string, args: string[]): Promise<string> {
  return new Promise((resolve, reject) => {
    let output = '';
    let errorOutput = '';
    
    const process = cp.spawn(command, args);
    
    process.stdout.on('data', (data: Buffer) => {
      output += data.toString();
    });
    
    process.stderr.on('data', (data: Buffer) => {
      errorOutput += data.toString();
    });
    
    process.on('close', (code: number) => {
      if (code === 0) {
        resolve(output);
      } else {
        reject(new Error(`Command failed with code ${code}: ${errorOutput}`));
      }
    });
    
    process.on('error', (err: Error) => {
      reject(err);
    });
  });
}

export function deactivate() {
  // Clean up resources
  if (diagnosticCollection) {
    diagnosticCollection.clear();
    diagnosticCollection.dispose();
  }
}
