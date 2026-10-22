<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let className: string = '';
  export let forId: string = '';
</script>

<label {forId} class={cn(className, 'block text-xs font-semibold text-gray-600 mb-1.5 uppercase tracking-wide', className)}><slot /></label>
