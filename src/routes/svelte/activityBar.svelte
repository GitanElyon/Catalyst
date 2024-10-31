<script lang="ts">
    import { getContext, onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    interface FileEntry {
        path: string;
        is_dir: boolean;
        children?: FileEntry[];
    }

    const workSpaceDir = getContext("workSpaceDir") as string;

    let files: FileEntry[] = [];

    async function loadFiles() {
        if (workSpaceDir) {
            try {
                const result = await invoke<FileEntry[]>("read_dir", { workspaceDir: workSpaceDir });
                console.log("Loaded files:", result); // Debugging output
                files = sortFiles(result);
            } catch (error) {
                console.error("Error reading directory:", error)
            }
        }
    }

    async function loadDirectory(path: string, parent: FileEntry) {
        try {
            const result = await invoke<FileEntry[]>("load_directory", { path });
            console.log("Loaded directory:", result); // Debugging output
            parent.children = sortFiles(result);
            // Trigger reactivity
            files = [...files];
        } catch (error) {
            console.error("Error loading directory:", error);
        }
    }

    function toggleFolder(event: Event, file: FileEntry) {
        const target = event.currentTarget as HTMLElement;
        const childrenContainer = target.nextElementSibling as HTMLElement;
        if (childrenContainer) {
            childrenContainer.classList.toggle('collapsed');
            console.log("Toggled folder:", file.path); // Debugging output
            if (!file.children || file.children.length === 0) {
                loadDirectory(file.path, file);
            }
        }
    }

    function getFileName(path: string, baseDir: string): string {
        const normalizedBaseDir = baseDir.replace(/\\/g, '/'); // Normalize base directory
        const normalizedPath = path.replace(/\\/g, '/'); // Normalize path
        return normalizedPath.replace(normalizedBaseDir, '').replace(/^\//, ''); // Remove base directory and leading slash
    }

    function sortFiles(files: FileEntry[]): FileEntry[] {
        return files.sort((a, b) => {
            if (a.is_dir && !b.is_dir) return -1;
            if (!a.is_dir && b.is_dir) return 1;
            return getFileName(a.path, workSpaceDir).localeCompare(getFileName(b.path, workSpaceDir));
        });
    }

    onMount(() => {
        loadFiles();
    });
</script>

<style>
    .file-explorer {
        background-color: #1e1e1e;
        color: white;
        border-radius: 15px;
        overflow-y: auto;
        height: 100%;
        font-family: Helvetica;
        padding-left: 10px;
    }
    .folder {
        font-weight: bold;
        cursor: pointer;
    }
    .file, .folder {
        margin-left: 20px;
    }
    .children {
        margin-left: 20px;
    }
    .collapsed {
        display: none;
    }
</style>

<div class="file-explorer">
    <h3>Workspace</h3>
    <div>
        {#each files as file}
            <div>
                {#if file.is_dir}
                    <div class="folder" on:click={(event) => toggleFolder(event, file)}>{getFileName(file.path, workSpaceDir)}</div>
                    <div class="children collapsed">
                        {#if file.children}
                            {#each file.children as child}
                                <div>
                                    {#if child.is_dir}
                                        <div class="folder" on:click={(event) => toggleFolder(event, child)}>{getFileName(child.path, workSpaceDir)}</div>
                                        <div class="children collapsed">
                                            {#if child.children}
                                                {#each child.children as grandchild}
                                                    <div>
                                                        {#if grandchild.is_dir}
                                                            <div class="folder" on:click={(event) => toggleFolder(event, grandchild)}>{getFileName(grandchild.path, workSpaceDir)}</div>
                                                            <div class="children collapsed"></div>
                                                        {:else}
                                                            <div class="file">{getFileName(grandchild.path, workSpaceDir)}</div>
                                                        {/if}
                                                    </div>
                                                {/each}
                                            {/if}
                                        </div>
                                    {:else}
                                        <div class="file">{getFileName(child.path, workSpaceDir)}</div>
                                    {/if}
                                </div>
                            {/each}
                        {/if}
                    </div>
                {:else}
                    <div class="file">{getFileName(file.path, workSpaceDir)}</div>
                {/if}
            </div>
        {/each}
    </div>
</div>