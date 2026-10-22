<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let className: string = '';
</script>

<div class={cn(className, 'text-sm font-semibold text-gray-600', className)}><slot /></div>
