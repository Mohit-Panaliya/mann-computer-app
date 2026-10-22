<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let className: string = '';
</script>

<div class={cn(className, 'flex flex-col space-y-1 p-6', className)}><slot /></div>
