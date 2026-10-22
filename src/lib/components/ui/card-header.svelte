<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let className: string = '';
</script>

<div class={cn(className, 'rounded-lg border bg-white p-6', className)}><slot /></div>
