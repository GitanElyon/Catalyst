<script lang="ts">
  import { onMount, setContext } from 'svelte';
  import { getCurrentWindow } from '@tauri-apps/api/window';
  import MainWindow from './window.svelte';

  let workSpaceDir = "C:/Projects/catalyst";
  //let workSpaceDir = null;
  
  setContext('workSpaceDir', workSpaceDir);

  let openFiles = [
    'src/routes/+page.svelte',
    'src/routes/window.svelte',
    'src/routes/editor.svelte',
    'src/routes/activityBar.svelte',
    'src/routes/panel.svelte',
    'src/routes/+layout.ts',
  ];


  const appWindow = getCurrentWindow();

  let showMenu = false;
  let isCursorOnMenu = false;

  let showWindowControls = false;
  let isCursorOnWindowControls = false;

  let showConsoleMenu = false;
  let isCursorOnConsoleMenu = false;

  let showSettingsMenu = false;
  let isCursorOnSettingsMenu = false;

  let showRightBar = false;
  let isCursorOnRightBar = false;

  let showLeftBar = false;
  let isCursorOnLeftBar = false;

  let showSearchBar = false;
  let isCursorOnSearchBar = false;

  function handleMouseMove(event: MouseEvent) {
    const menuActivationWidth = window.innerWidth * 0.2;
    const windowControlsActivationWidth = window.innerWidth * 0.1;
    const consoleMenuActivationWidth = window.innerWidth * 0.2;
    const settingsMenuActivationWidth = window.innerWidth * 0.2;
    const rightBarActivationHeight = window.innerHeight * 0.4;
    const leftBarActivationHeight = window.innerHeight * 0.6;
    const searchBarActivationWidth = window.innerWidth * 0.4;

    if (event.clientX < menuActivationWidth && event.clientY < 20 && !isCursorOnMenu) {
      showMenu = true;
    } else if (!isCursorOnMenu) {
      showMenu = false;
    }

    if (event.clientX > window.innerWidth - windowControlsActivationWidth && event.clientY < 20 && !isCursorOnWindowControls) {
      showWindowControls = true;
    } else if (!isCursorOnWindowControls) {
      showWindowControls = false;
    }

    if (event.clientX < consoleMenuActivationWidth && event.clientY > window.innerHeight - 20 && !isCursorOnConsoleMenu) {
      showConsoleMenu = true;
    } else if (!isCursorOnConsoleMenu) {
      showConsoleMenu = false;
    }

    if (event.clientX > window.innerWidth - settingsMenuActivationWidth && event.clientY > window.innerHeight - 20 && !isCursorOnSettingsMenu) {
      showSettingsMenu = true;
    } else if (!isCursorOnSettingsMenu) {
      showSettingsMenu = false;
    }

    if (event.clientX > window.innerWidth - 20 && event.clientY < rightBarActivationHeight && !isCursorOnRightBar) {
      showRightBar = true;
    } else if (!isCursorOnRightBar) {
      showRightBar = false;
    }

    if (event.clientX < 20 && event.clientY < leftBarActivationHeight && !isCursorOnLeftBar) {
      showLeftBar = true;
    } else if (!isCursorOnLeftBar) {
      showLeftBar = false;
    }

    const topCenterStartX = (window.innerWidth - searchBarActivationWidth) / 2;
    const topCenterEndX = (window.innerWidth + searchBarActivationWidth) / 2;

    if (event.clientY < 20 && event.clientX > topCenterStartX && event.clientX < topCenterEndX && !isCursorOnSearchBar) {
      showSearchBar = true;
    } else if (!isCursorOnSearchBar) {
      showSearchBar = false;
    }
  }

  // just some failed testing to make code cleaner
  function mouseEnterMenu(handle: string) {
    switch (handle) {
      case 'menu':
        isCursorOnMenu = true;
        showMenu = true;
        break;
      case 'windowControls':
        isCursorOnWindowControls = true;
        showWindowControls = true;
        break;
      case 'consoleMenu':
        isCursorOnConsoleMenu = true;
        showConsoleMenu = true;
        break;
      case 'settingsMenu':
        isCursorOnSettingsMenu = true;
        showSettingsMenu = true;
        break;
      case 'rightBar':
        isCursorOnRightBar = true;
        showRightBar = true;
        break;
      case 'leftBar':
        isCursorOnLeftBar = true;
        showLeftBar = true;
        break;
      case 'searchBar':
        isCursorOnSearchBar = true;
        showSearchBar = true;
        break;
      default:
        console.error('Invalid handle');
        break;
    }

  }

  function mouseLeaveMenu(handle: string) {
    switch (handle) {
      case 'menu':
        isCursorOnMenu = false;
        showMenu = false;
        break;
      case 'windowControls':
        isCursorOnWindowControls = false;
        showWindowControls = false;
        break;
      case 'consoleMenu':
        isCursorOnConsoleMenu = false;
        showConsoleMenu = false;
        break;
      case 'settingsMenu':
        isCursorOnSettingsMenu = false;
        showSettingsMenu = false;
        break;
      case 'rightBar':
        isCursorOnRightBar = false;
        showRightBar = false;
        break;
      case 'leftBar':
        isCursorOnLeftBar = false;
        showLeftBar = false;
        break;
      case 'searchBar':
        isCursorOnSearchBar = false;
        showSearchBar = false;
        break;
      default:
        console.error('Invalid handle');
        break;
    }
  }
  
  // because, this shit is a mess
  function handleMouseEnter() {
    isCursorOnMenu = true;
    showMenu = true;
  }

  function handleMouseLeave() {
    isCursorOnMenu = false;
    showMenu = false;
  }

  function handleWindowControlsMouseEnter() {
    isCursorOnWindowControls = true;
    showWindowControls = true;
  }

  function handleWindowControlsMouseLeave() {
    isCursorOnWindowControls = false;
    showWindowControls = false;
  }

  function handleConsoleMenuMouseEnter() {
    isCursorOnConsoleMenu = true;
    showConsoleMenu = true;
  }

  function handleConsoleMenuMouseLeave() {
    isCursorOnConsoleMenu = false;
    showConsoleMenu = false;
  }

  function handleSettingsMenuMouseEnter() {
    isCursorOnSettingsMenu = true;
    showSettingsMenu = true;
  }

  function handleSettingsMenuMouseLeave() {
    isCursorOnSettingsMenu = false;
    showSettingsMenu = false;
  }

  function handleRightBarMouseEnter() {
    isCursorOnRightBar = true;
    showRightBar = true;
  }

  function handleRightBarMouseLeave() {
    isCursorOnRightBar = false;
    showRightBar = false;
  }

  function handleLeftBarMouseEnter() {
    isCursorOnLeftBar = true;
    showLeftBar = true;
  }

  function handleLeftBarMouseLeave() {
    isCursorOnLeftBar = false;
    showLeftBar = false;
  }

  function handleSearchBarMouseEnter() {
    isCursorOnSearchBar = true;
    showSearchBar = true;
  }

  function handleSearchBarMouseLeave() {
    isCursorOnSearchBar = false;
    showSearchBar = false;
  }

  function minimizeWindow() {
    appWindow.minimize();
  }

  function maximizeWindow() {
    appWindow.toggleMaximize();
  }

  function closeWindow() {
    appWindow.close();
  }

  onMount(() => {
    window.addEventListener('mousemove', handleMouseMove);
    return () => {
      window.removeEventListener('mousemove', handleMouseMove);
    };
  });
</script>

<style>
  :global(body) {
    margin: 0;
    background-color: hsl(0, 0%, 9%);
    color: white;
    height: 100vh;
    overflow: hidden;
  }

  .menu-container {
    position: fixed;
    text-align: left;
    z-index: 9999;
  }

  .menu {
    display: inline-block;
    background-color: rgba(19, 19, 19, 0.738);
    color: white;
    padding: 10px;
    /* box-shadow: 0 2px 5px rgba(0, 0, 0, 0.5); */
    border-radius: 10px; 
    margin: 0 10px;
    font-size: 0.9em;
  }


  .menu-item {
    display: inline-block;
    margin-right: 15px;
    cursor: pointer;
    font-family: Helvetica;
    font-weight: 500;
  }

  .menu-item:last-child {
    margin-right: 0;
  }

  .main-menu-container {
    top: -100px;
    left: 0;
    width: 100%;
    transition: top 0.3s ease;
  }

  .main-menu-container.show {
    top: 10px;
  }

  .logo {
    height: 20px;
    vertical-align: middle;
    margin-right: 15px;
    cursor: pointer;
  }

  .window-controls-container {
    top: -100px;
    right: 0;
    transition: top 0.3s ease;
  }

  .window-controls-container.show {
    top: 10px;
  }

  .console-menu-container {
    bottom: -100px;
    left: 0;
    width: 100%;
    transition: bottom 0.3s ease;
  }

  .console-menu-container.show {
    bottom: 10px;
  }

  .settings-menu-container {
    bottom: -100px;
    right: 0;
    transition: bottom 0.3s ease;
  }


  .settings-menu-container.show {
    bottom: 10px;
  }

  .right-bar-container {
    top: 0;
    right: -100px;
    width: 100px;
    transition: right 0.3s ease;
    text-align: center;
    margin-top: 50px;
  }

  .right-bar {
    display: flex;
    flex-direction: column;
    
  }

  .right-bar-container.show {
    right: 0;
  }

  .right-bar-item {
    margin: 10px 0;
  }

  .left-bar-container {
    top: 0;
    left: -150px;
    width: 150px; 
    transition: left 0.3s ease;
    text-align: left;
    margin-top: 50px;
    overflow: hidden;
  }

  .left-bar {
    display: flex;
    flex-direction: column;
    margin: 10px;
    
  }

  .left-bar-container.show {
    left: 0;
  }

  .left-bar-item {
    margin: 10px 0;
  }

  .search-bar-container {
    position: fixed;
    top: -100px;
    left: 50%;
    transform: translateX(-50%);
    width: 400px;
    transition: top 0.3s ease;
    text-align: center;
  }

  .search-bar-container.show {
    top: 10px;
  }

  .search-bar-item {
    cursor: pointer;
    font-family: Helvetica;
    font-weight: 500;
    background-color: rgb(19, 19, 19);
    border-radius: 5px;
    padding: 2px;
    border: none;
    user-select: none;
    outline: none;
    color: white;
  }
</style>

<div>
  <div class="menu-container main-menu-container {showMenu ? 'show' : ''}">
    <div class="menu main-menu" role="dialog" aria-modal="true" on:mouseenter={handleMouseEnter} on:mouseleave={handleMouseLeave}>
      <a href="https://github.com/GitanElyon/Catalyst" target="_blank">
        <img src="src\routes\assets\app-icon.png" alt="App Logo" class="logo" />
      </a>
      <span class="menu-item main-menu-item">File</span>
      <span class="menu-item main-menu-item">Edit</span>
      <span class="menu-item main-menu-item">View</span>
      <span class="menu-item main-menu-item">Run</span>
      <span class="menu-item main-menu-item">Terminal</span>
      <span class="menu-item main-menu-item">Help</span>
    </div>
  </div>

  <div class="menu-container window-controls-container {showWindowControls ? 'show' : ''}">
    <div class="menu window-controls" role="dialog" aria-modal="true" on:mouseenter={handleWindowControlsMouseEnter} on:mouseleave={handleWindowControlsMouseLeave}>
      <span class="menu-item window-control-item" on:click={minimizeWindow}>-</span>
      <span class="menu-item window-control-item" on:click={maximizeWindow}>O</span>
      <span class="menu-item window-control-item" on:click={closeWindow}>X</span>
      <!--
      <button class="window-control-item" on:click={minimizeWindow} on:keydown={(e) => e.key === 'Enter' && minimizeWindow()} aria-label="Minimize window">-</button>
      <button class="window-control-item" on:click={maximizeWindow} on:keydown={(e) => e.key === 'Enter' && maximizeWindow()} aria-label="Maximize window">O</button>
      <button class="window-control-item" on:click={closeWindow} on:keydown={(e) => e.key === 'Enter' && closeWindow()} aria-label="Close window">X</button>
      -->
    </div>
  </div>

  <div class="menu-container console-menu-container {showConsoleMenu ? 'show' : ''}">
    <div class="menu console-menu" role="dialog" aria-modal="true" on:mouseenter={handleConsoleMenuMouseEnter} on:mouseleave={handleConsoleMenuMouseLeave}>
      <span class="menu-item console-menu-item">Terminal</span>
      <span class="menu-item console-menu-item">Console</span>
      <span class="menu-item console-menu-item">Ports</span>
      <span class="menu-item console-menu-item">Debug</span>
      <span class="menu-item console-menu-item">Problems</span>
    </div>
  </div>

  <div class="menu-container settings-menu-container {showSettingsMenu ? 'show' : ''}">
    <div class="menu settings-menu" role="dialog" aria-modal="true" on:mouseenter={handleSettingsMenuMouseEnter} on:mouseleave={handleSettingsMenuMouseLeave}>
      <span class="menu-item settings-menu-item">Settings</span>
      <span class="menu-item settings-menu-item">Extensions</span>
      <span class="menu-item settings-menu-item">Themes</span>
      <span class="menu-item settings-menu-item">Shortcuts</span>
    </div>
  </div>

  <div class="menu-container right-bar-container {showRightBar ? 'show' : ''}" role="complementary" on:mouseenter={handleRightBarMouseEnter} on:mouseleave={handleRightBarMouseLeave}>
    <div class="menu right-bar" role="navigation">
      <span class="menu-item right-bar-item">Files</span>
      <span class="menu-item right-bar-item">Search</span>
      <span class="menu-item right-bar-item">Git</span>
      <span class="menu-item right-bar-item">Debug</span>
      <span class="menu-item right-bar-item">Copilot</span>
      <span class="menu-item right-bar-item">Docker</span>
      <span class="menu-item right-bar-item">Preview</span>
    </div>
  </div>

  <div class="menu-container left-bar-container {showLeftBar ? 'show' : ''}" role="navigation" on:mouseenter={handleLeftBarMouseEnter} on:mouseleave={handleLeftBarMouseLeave}>
    <div class="menu left-bar">
      {#each openFiles as file}
        <span class="menu-item left-bar-item">{file.split('/').pop()}</span>
      {/each}
    </div>
  </div>

  <div class="menu-container search-bar-container {showSearchBar ? 'show' : ''}" role="search" on:mouseenter={handleSearchBarMouseEnter} on:mouseleave={handleSearchBarMouseLeave}>
    <div class="menu search-bar">
      <input type="text" placeholder="Search..." class="search-bar-item" />
    </div>
  </div>

  <MainWindow />
</div>