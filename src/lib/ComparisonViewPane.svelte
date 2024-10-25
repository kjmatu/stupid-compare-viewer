<script lang="ts">
    import ComparisonImage from '$lib/ComparisonImage.svelte';
    import FilePathList from '$lib/FilePathList.svelte';


    const {imagePath, alt, filePaths} = $props<{imagePath: string, alt: string, filePaths: Array<string>}>();
</script>

<ComparisonImage imagePath={imagePath} alt={alt} />
<FilePathList filePaths={filePaths} />