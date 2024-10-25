<script lang="ts">
    import Image from '../lib/Image.svelte';
    import FilePathList from '$lib/FilePathList.svelte';
    import { invoke } from '@tauri-apps/api/tauri';

	import type { OpenedDirInfo } from './../../src-tauri/bindings/OpenedDirInfo';


    let openedDirInfo: OpenedDirInfo = $state(
        {
            "imageFilePaths": [],
        }
    )
    async function openImage() {
        openedDirInfo = await invoke('open_image')
    }


</script>

<button onclick={openImage}>OPEN</button>

<Image leftImagePath={openedDirInfo.imageFilePaths[0]} rightImagePath={openedDirInfo.imageFilePaths[1]} />

<FilePathList filePaths={openedDirInfo.imageFilePaths} />