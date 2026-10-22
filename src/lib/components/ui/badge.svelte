<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let className: string = '';
  export let variant: 'default' | 'secondary' | 'destructive' | 'outline' = 'default';

  const variants = {
    default: 'bg-blue-600 text-white hover:bg-blue-700',
    secondary: 'bg-gray-100 text-gray-900 hover:bg-gray-200',
    destructive: 'bg-red-600 text-white hover:bg-red-700',
    outline: 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50',
  };
</script>

<span class={cn(className, 'inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-semibold', variants[variant], className)}><slot /></span>
