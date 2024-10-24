<script lang="ts">
    let filePaths = $props<{filePaths: Array<string>}>();
    console.log('FilePathListコンポーネント内のfilePaths:', filePaths);
</script>

<style>
    ul {
        list-style-type: none;
        padding: 0;
    }

    li {
        padding: 8px;
        border-bottom: 1px solid #ccc;
        cursor: pointer;
    }

    li:hover {
        background-color: #f0f0f0;
    }
</style>

{#if filePaths.length > 0}
    <ul>
        {#each filePaths as path}
        <li>{path}</li>
        {/each}
    </ul>
{:else}
    <p>ファイルパスがありません。</p>
{/if}