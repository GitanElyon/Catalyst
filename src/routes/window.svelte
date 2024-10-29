<script lang="ts">
  import CodeEditor from './editor.svelte';
  import FileExplorer from './activityBar.svelte';
  import Console from './panel.svelte';

  // Initial sizes as percentages
  let codeEditorWidth: number = 70; // Width of the code editor
  let consoleHeight: number = 30; // Height of the console

  // Handle resizing for the width of the Code Editor and File Explorer
  function handleHorizontalResize(event: MouseEvent): void {
    const startX = event.clientX;
    const startWidth = codeEditorWidth;

    const mouseMoveHandler = (e: MouseEvent): void => {
      const newWidth = startWidth + ((e.clientX - startX) / window.innerWidth) * 100;
      if (newWidth >= 10 && newWidth <= 90) { // Limits for resizing
        codeEditorWidth = newWidth;
      }
    };

    const mouseUpHandler = (): void => {
      document.removeEventListener('mousemove', mouseMoveHandler);
      document.removeEventListener('mouseup', mouseUpHandler);
    };

    document.addEventListener('mousemove', mouseMoveHandler);
    document.addEventListener('mouseup', mouseUpHandler);
  }

  // Handle resizing for the height of the Console
  function handleVerticalResize(event: MouseEvent): void {
    const startY = event.clientY;
    const startHeight = consoleHeight;

    const mouseMoveHandler = (e: MouseEvent): void => {
      const newHeight = startHeight + ((startY - e.clientY) / window.innerHeight) * 100;
      if (newHeight >= 10 && newHeight <= 90) { // Limits for resizing
        consoleHeight = newHeight;
      }
    };

    const mouseUpHandler = (): void => {
      document.removeEventListener('mousemove', mouseMoveHandler);
      document.removeEventListener('mouseup', mouseUpHandler);
    };

    document.addEventListener('mousemove', mouseMoveHandler);
    document.addEventListener('mouseup', mouseUpHandler);
  }
</script>

<style>
  .main-container {
    display: grid;
    grid-template-areas:
      "code-editor file-explorer"
      "console file-explorer";
    grid-template-columns: var(--code-editor-width) calc(100vw - var(--code-editor-width) - 5px);
    grid-template-rows: calc(100vh - var(--console-height) - 5px) var(--console-height);
    gap: 5px;
    height: 100vh;
    position: relative; 
    z-index: 1;
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

  .resizer {
    cursor: ew-resize;
    width: 5px; /* Width of the horizontal resizer */
    background-color: #0000005a; /* Color of the resizer */
    height: 100%; /* Full height for resizer */
    position: absolute; /* Positioning */
    left: var(--code-editor-width); /* Align next to the code editor */
    top: 0;
    z-index: 10; /* Ensure the resizer is above other elements */
  }

  .vertical-resizer {
    cursor: ns-resize;
    height: 5px; /* Height of the vertical resizer */
    background-color: #0000005a; /* Color of the vertical resizer */
    width: var(--code-editor-width); /* Full width for resizer */
    position: absolute; /* Positioning */
    bottom: calc(var(--console-height)); /* Align above the console */
    left: 0;
    z-index: 10; /* Ensure the resizer is above other elements */
  }
</style>

<!-- Update CSS variables dynamically in the style attribute -->
<div class="main-container" style="--code-editor-width: {codeEditorWidth}vw; --console-height: {consoleHeight}vh;">
  <div class="code-editor">
    <CodeEditor />
  </div>
  <div class="file-explorer">
    <FileExplorer />
  </div>
  <div class="console" style="height: {consoleHeight}vh;">
    <Console />
  </div>
  <div class="resizer" on:mousedown={handleHorizontalResize}></div>
  <div class="vertical-resizer" on:mousedown={handleVerticalResize}></div>
</div>
