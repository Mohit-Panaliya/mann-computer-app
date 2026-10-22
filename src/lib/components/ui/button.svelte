<script lang="ts">
  import { cn } from '$lib/utils/utils';

  export let variant: 'default' | 'destructive' | 'outline' | 'secondary' | 'ghost' | 'link' = 'default';
  export let size: 'default' | 'sm' | 'lg' | 'icon' = 'default';
  export let className: string = '';
  export let disabled: boolean = false;

  const variants = {
    default: 'bg-blue-600 text-white hover:bg-blue-700 focus:ring-2 focus:ring-blue-500 shadow-sm',
    destructive: 'bg-red-600 text-white hover:bg-red-700 focus:ring-2 focus:ring-red-500 shadow-sm',
    outline: 'border border-gray-300 bg-white text-gray-700 hover:bg-gray-50 focus:ring-2 focus:ring-blue-500',
    secondary: 'bg-gray-100 text-gray-900 hover:bg-gray-200 focus:ring-gray-500 shadow-sm',
    ghost: 'hover:bg-gray-100 hover:text-gray-900 focus:ring-gray-500',
    link: 'text-primary-underline-offset-4 hover:underline',
  };

  const sizes = {
    default: 'h-10 px-4 py-2',
    sm: 'h-9 px-3',
    lg: 'h-11 px-8',
    icon: 'h-10 w-10',
  };
</script>

<button
  class={cn(
    'inline-flex items-center justify-center rounded-xl text-sm font-medium transition-all duration-200 disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:ring-2 focus:ring-offset-2',
    variants[variant],
    sizes[size],
    className
  )}
  {disabled}
>
  <slot />
</button>
