<script>
    import CodeEditor from './editor.svelte';
    import FileExplorer from './activityBar.svelte';
    import Console from './panel.svelte';
  </script>
  
  <style>
    .main-container {
      padding: 5px;
      display: grid;
      grid-template-areas:
        "code-editor file-explorer"
        "console file-explorer";
      grid-template-columns: 70vw calc(30vw - 15px);
      grid-template-rows: 70vh calc(30vh - 15px);
      gap: 5px
    }
  
    .file-explorer {
      grid-area: file-explorer;
    }
  
    .code-editor {
      grid-area: code-editor;
    }
  
    .console {
      grid-area: console;
    }
  </style>
  
  <div class="main-container">
    <div class="code-editor">
      <CodeEditor />
    </div>
    <div class="file-explorer">
      <FileExplorer />
    </div>
    <div class="console">
      <Console />
    </div>
  </div>