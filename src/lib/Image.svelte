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
    <p>{loadedImageData.filePath}</p>

    <div class="image-container">
        <div class="image-wrapper">
            <img src={`${convertFileSrc(loadedImageData.filePath)}`} alt="Left">
        </div>
        <div class="image-wrapper">
            <img src={`${convertFileSrc(loadedImageData.filePath)}`} alt="Right">
        </div>
    </div>
</div> 

<style>
    .image-container {
        display: flex;
        justify-content: space-between;
        flex-wrap: wrap; /* 画面が狭い場合に画像を縦に並べるために使用 */
    }

    .image-wrapper {
        flex: 1;
        margin: 0 10px; /* 画像の間に余白を追加 */
        position: relative;
        aspect-ratio: 16/9; /* アスペクト比を指定 */
        max-width: 48%; /* 初期設定: 幅の最大値を48%に設定 */
    }

    .image-wrapper img {
        position: absolute;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        object-fit: contain; /* アスペクト比を保ちながら、画像をコンテナに収める */
    }

    /* 画面幅が600px以下の場合に画像の幅を100%にする */
    @media (max-width: 600px) {
        .image-wrapper {
            max-width: 100%;
            margin-bottom: 20px; /* 縦に並べた時に下部に余白を追加 */
        }
    }

    /* 画面幅が900px以下の場合に画像の幅を48%から70%にする */
    @media (max-width: 900px) {
        .image-wrapper {
            max-width: 70%;
        }
    }
</style>