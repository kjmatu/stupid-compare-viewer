<script lang="ts">
    import ComparisonViewPane from '$lib/ComparisonViewPane.svelte';
</script>

<div class="comparison-container">
    <ComparisonViewPane alt="Left" />
    <ComparisonViewPane alt="Right" />
</div>


<style>
    .comparison-container {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        gap: 20px; /* ペイン間の間隔を調整 */
    }

    :global(.comparison-container > *) {
        flex: 1; /* 両方のペインが等しい幅を持つように */
    }
</style>