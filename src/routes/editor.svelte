<script lang="ts">
    let code = `# Write your code here...`;
  
    import { getContext } from "svelte";
  
    // Get the context
    const workSpaceDir = getContext("workSpaceDir");
  
    // Check if the variable is null
    $: isWorkSpaceDirNull = workSpaceDir === null;
  </script>
  
  <div class="code-editor">
    {#if isWorkSpaceDirNull}
      <div class="centered-content">
        <img src="src\routes\assets\CatalystBackground.png" alt="Catalyst" />
        <div class="shortcuts">
            <p>All Shortcuts: <kbd>ctrl</kbd> + <kbd>shift</kbd> + <kbd>S</kbd></p>
            <p>Open Workspace: <kbd>ctrl</kbd> + <kbd>O</kbd></p>
            <p>Recent Files: <kbd>ctrl</kbd> + <kbd>R</kbd></p>
          
        </div>
      </div>
    {:else}
      <textarea bind:value={code}></textarea>
    {/if}
  </div>
  
  <style>
    .code-editor {
      background-color: #1e1e1e;
      color: white;
      border-radius: 15px;
      overflow: auto;
      height: 100%;
      display: flex;
      justify-content: center;
      align-items: center;
      flex-direction: column;
      font-family: monospace;
    }
  
    .code-editor textarea {
      width: 100%;
      height: 100%;
      background-color: transparent;
      border: none;
      color: white;
      resize: none;
      padding: 10px;
      font-family: monospace;
      font-size: 1em;
    }
  
    .code-editor textarea:focus {
      outline: none;
    }
  
    .centered-content {
      display: flex;
      flex-direction: column;
      justify-content: center;
      align-items: center;
      height: 100%;
    }
  
    .centered-content img {
      max-width: 80%;
      max-height: 80%;
    }
  
    .shortcuts {
      margin-top: 20px;
      text-align: center;
    }
  
    .shortcuts p {
      margin: 5px 0;
    }
  
    .shortcuts kbd {
      background-color: #333;
      border-radius: 3px;
      padding: 2px 4px;
      font-family: monospace;
      font-size: 0.9em;
      color: white;
    }
  
    .code-editor::-webkit-scrollbar {
      display: none;
    }
  </style>