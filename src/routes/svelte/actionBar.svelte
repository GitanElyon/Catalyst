<script lang="ts">
    import Debug from "./actionBar/debug.svelte";
    import Terminal from "./actionBar/terminal.svelte";
    import Problems from "./actionBar/problems.svelte";
    import Ports from "./actionBar/ports.svelte";

    let log = "PS C:\\Projects\\Catalyst> _";
  </script>
  
  <style>
    .console {
      background-color: #1e1e1e;
      color: white;
      border-radius: 15px;
      overflow-y: auto;
      height: 100%;
      padding-left: 10px;
      font-family: monospace;
    }
  </style>
  <div class="console">
    <!--<p>{log}</p>-->
    <Terminal />
  </div>
  