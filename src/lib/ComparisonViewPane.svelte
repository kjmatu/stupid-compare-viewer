<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri';

    import type { OpenedDirInfo } from './../../src-tauri/bindings/OpenedDirInfo';

    import ComparisonImage from '$lib/ComparisonImage.svelte';
    import FilePathList from '$lib/FilePathList.svelte';

    let openedDirInfo: OpenedDirInfo = $state(
        {
            "imageFilePaths": [],
        }
    )
    async function openImage() {
        openedDirInfo = await invoke('open_image')
    }

    const {alt} = $props<{alt: string}>();
</script>

<div class="comparison-view-pane">
    <button onclick={openImage}>Open</button>
    <ComparisonImage imagePath={openedDirInfo.imageFilePaths[0]} alt={alt} />
    <FilePathList filePaths={openedDirInfo.imageFilePaths} />
</div>

<style>
    .comparison-view-pane {
        display: flex;
        flex-direction: column;
        height: 100%;
    }
</style>