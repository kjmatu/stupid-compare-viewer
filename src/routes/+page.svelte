<script lang="ts">
    import ComparisonViewPane from '$lib/ComparisonViewPane.svelte';

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
<ComparisonViewPane imagePath={openedDirInfo.imageFilePaths[0]} alt="Left" filePaths={openedDirInfo.imageFilePaths} />
