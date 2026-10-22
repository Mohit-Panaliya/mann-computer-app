<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let open: boolean = false;
  export let className: string = '';

  function toggle() {
    open = !open;
  }

  export { toggle };
</script>

{#if open}
  <div class={cn('fixed inset-0 z-50 bg-black/80 backdrop-blur-sm', className)} on:click={() => (open = false)} role="dialog" aria-modal="true" />
  <div class="fixed z-[60] left-[50%] top-[50%] -translate-x-[50%] -translate-y-[50%] w-full max-w-md border border-gray-200 bg-white p-6 shadow-lg outline-none rounded-xl">
    <slot />
  </div>
{/if}
