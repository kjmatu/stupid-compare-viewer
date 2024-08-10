<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let selectedFileName = $state("");
    let imageData = $state("");
    async function openImage() {
        imageData = await invoke('open_image')
    }
</script>

<div>
    <button onclick={openImage}>OPEN</button>
    <p>{selectedFileName}</p>
    <img src={`data:image/jpeg;base64,${imageData.base64Data}`} alt="src">
</div>