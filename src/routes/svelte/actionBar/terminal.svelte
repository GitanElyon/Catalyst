<script lang="ts">
    import { onMount } from 'svelte';
    import { Terminal } from 'xterm';
    import 'xterm/css/xterm.css';
  
    let terminalContainer: HTMLDivElement;
    const terminal = new Terminal();
  
    type CommandFunction = (args?: string[]) => string;
  
    const commands: { [key: string]: CommandFunction } = {
      help: () => {
        return 'Available commands: help, clear, echo [text]';
      },
      clear: () => {
        terminal.clear();
        return '';
      },
      echo: (args: string[] = []) => {
        return args.join(' ');
      }
    };
  
    let input = ''; // Move input variable outside to keep state
  
    onMount(() => {
      terminal.open(terminalContainer);
      terminal.writeln('Welcome to Catalyst! (version 0.1.2)');
      terminal.writeln('Type "help" for a list of commands.');
      terminal.write('\r\n$ '); // Prompt at the start
  
      terminal.onData((data: string) => {
        const char = data.charCodeAt(0);
  
        // Handle Enter
        if (char === 13) {
          terminal.writeln(''); // Move to next line
          handleCommand(input);
          input = ''; // Reset input after handling command
          terminal.write('\r\n$ '); // Show prompt again
        } 
        // Handle Backspace
        else if (char === 127) {
          if (input.length > 0) {
            input = input.slice(0, -1); // Remove last character from input
            terminal.write('\b \b'); // Visual backspace
          }
        } 
        // Handle other inputs
        else {
          input += data; // Append new data to input
          terminal.write(data); // Display the character
        }
      });
    });
  
    function handleCommand(input: string) {
      const [command, ...args] = input.trim().split(' ');
      const output = commands[command]?.(args) ?? `Command not found: ${command}`;
      terminal.writeln(output);
    }
  </script>
  
  <style>
    .terminal-container {
      width: 100%;
      height: 100%;
      background-color: black;
      color: white;
    }
  </style>
  
  <div bind:this={terminalContainer} class="terminal-container"></div>
  