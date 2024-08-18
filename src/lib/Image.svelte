<script lang="ts">
    import { invoke, convertFileSrc } from '@tauri-apps/api/tauri';
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

<div>
    <button onclick={openImage}>OPEN</button>
    <p>width={loadedImageData.width}, height={loadedImageData.height} {loadedImageData.filePath}</p>
    <img src={`${convertFileSrc(loadedImageData.filePath)}`} alt="selected img">
</div> 