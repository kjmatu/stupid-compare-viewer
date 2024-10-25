<script lang="ts">
    import { convertFileSrc } from '@tauri-apps/api/tauri';

    const {imagePath, alt} = $props<{imagePath: string, alt: string}>(); 
</script>

<div>
    <div class="image-wrapper">
        <p class="file-path">{imagePath}</p>
        <img src={`${convertFileSrc(imagePath)}`} alt={alt}>
    </div>
</div> 

<style>
    .image-wrapper {
        flex: 1;
        margin: 0 10px; /* 画像の間に余白を追加 */
        position: relative;
        aspect-ratio: 16/9; /* アスペクト比を指定 */
        max-width: 48%; /* 初期設定: 幅の最大値を48%に設定 */
    }

    .file-path {
        margin: 0;
        font-size: 1em;
        word-break: break-all; /* ファイルパスが長い場合に折り返す */
    }
    .image-wrapper img {
        margin-top: 10px; /* ファイルパスと画像の間に余白を追加 */
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