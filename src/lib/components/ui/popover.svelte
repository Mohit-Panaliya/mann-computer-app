<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let open: boolean = false;
  export let className: string = '';
</script>

{#if open}
  <div class={cn('fixed inset-0 z-50 bg-black/80 backdrop-blur-sm', className)} on:click={() => (open = false)} role="dialog" aria-modal="true" />
  <div class="fixed z-[60] w-full max-w-md" role="document">
    <div class="relative left-[50%] top-[50%] -translate-x-[50%] -translate-y-[50%] w-full max-w-lg border border-gray-200 bg-white p-6 shadow-lg outline-none rounded-xl">
      <button on:click={() => (open = false)} class="absolute right-4 top-4 rounded-sm opacity-70 ring-offset-background transition-opacity hover:opacity-100 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2 disabled:pointer-events-none data-[state=open]:bg-gray-100 data-[state=open]:text-gray-600">
        <span class="sr-only">Close</span>
        <svg class="h-4 w-4" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor"><path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293-4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" /></svg>
      </button>
      <slot />
    </div>
  </div>
{/if}
