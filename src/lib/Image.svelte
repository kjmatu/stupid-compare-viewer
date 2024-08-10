<script>
    import { invoke } from '@tauri-apps/api/tauri'

    let selectedFileName = $state("");
    async function openImage() {
        selectedFileName = await invoke('open_image')
    }
</script>

<div>
    <button onclick={openImage}>OPEN</button>
    <p>{selectedFileName}</p>
</div>