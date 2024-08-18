<script lang="ts">
    import Image from '../lib/Image.svelte';
    import { invoke } from '@tauri-apps/api/tauri';

	import type { LoadedImageData } from './../../src-tauri/bindings/LoadedImageData';


    let loadedImageData: LoadedImageData = $state(
        {
            "filePath": "",
        }
    )
    async function openImage() {
        loadedImageData = await invoke('open_image')
    }


</script>

<button onclick={openImage}>OPEN</button>

<Image leftImagePath={loadedImageData.filePath} rightImagePath={loadedImageData.filePath} />