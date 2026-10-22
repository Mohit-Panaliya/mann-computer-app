<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let className: string = '';
</script>

<input
  class={cn(className, 
    'block w-full px-4 py-2.5 text-sm border border-gray-300 rounded-xl bg-white focus:ring-2 focus:ring-blue-500 focus:border-blue-500 outline-none transition-all',
    className
  )}
/>