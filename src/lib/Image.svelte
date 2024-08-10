<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import type { LoadedImageData } from './../../src-tauri/bindings/LoadedImageData';

    let loadedImageData: LoadedImageData = $state(
        {
            "filePath": "",
            "base64Data": "",
            "height": 0,
            "width": 0
        }
    )
    async function openImage() {
        loadedImageData = await invoke('open_image')
    }
</script>

<div>
    <button onclick={openImage}>OPEN</button>
    <p>width={loadedImageData.width}, height={loadedImageData.height} {loadedImageData.filePath}</p>
    <img src={`data:image/jpeg;base64,${loadedImageData.base64Data}`} alt="src">
</div> 