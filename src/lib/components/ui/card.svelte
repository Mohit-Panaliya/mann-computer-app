<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let className: string = '';
</script>

<div class={cn(className, 'rounded-xl border border-gray-200 bg-white text-gray-900 shadow-sm', className)}><slot /></div>
