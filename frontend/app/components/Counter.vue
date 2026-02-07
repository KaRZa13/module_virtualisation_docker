<template>
  <div class="relative group">
    <div class="absolute -inset-1 bg-linear-to-r from-pink-600 to-purple-600 rounded-2xl blur opacity-25 group-hover:opacity-75 transition duration-1000 group-hover:duration-200"></div>
    <section class="relative bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 p-8 rounded-2xl shadow-2xl flex flex-col gap-6 items-center w-80 transform transition-all">
      <div class="space-y-2 text-center">
        <span class="text-slate-500 dark:text-slate-400 text-xs font-bold uppercase tracking-widest">Valeur actuelle</span>
        <p class="text-8xl text-transparent bg-clip-text bg-linear-to-br from-pink-500 to-violet-600 drop-shadow-sm font-counter leading-tight py-2">
          {{ counter }}
        </p>
      </div>
      <div class="grid grid-cols-2 gap-4 w-full">
        <button 
          class="py-3 px-6 bg-indigo-600 hover:bg-indigo-500 text-white rounded-xl shadow-lg shadow-indigo-500/30 font-bold transition-all transform active:scale-95 flex justify-center items-center" 
          @click="increment"
        >
          Ajouter
        </button>
        <button 
          class="py-3 px-6 bg-slate-100 dark:bg-slate-800 hover:bg-slate-200 dark:hover:bg-slate-700 text-slate-700 dark:text-slate-200 rounded-xl font-medium transition-all transform active:scale-95 border border-slate-200 dark:border-slate-700" 
          @click="reset"
        >
          Reset
        </button>  
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
const config = useRuntimeConfig()
const apiBase = config.public.apiBase
const counter = ref(0)

// Fetch initial value
onMounted(async () => {
  try {
    const data = await $fetch<{ value: number }>(`${apiBase}/counter`)
    counter.value = data.value
  } catch (e) {
    console.error('Failed to fetch counter', e)
  }
})

async function increment() {
  try {
    const data = await $fetch<{ value: number }>(`${apiBase}/increment`, { method: 'POST' })
    counter.value = data.value
  } catch (e) {
    console.error('Failed to increment', e)
  }
}

async function reset() {
  try {
    const data = await $fetch<{ value: number }>(`${apiBase}/reset`, { method: 'POST' })
    counter.value = data.value
  } catch (e) {
    console.error('Failed to reset', e)
  }
}
</script>

<style scoped>
.font-counter {
  font-family: 'Luckiest Guy', cursive;
}
button {
  font-family: 'Poppins', sans-serif;
}
</style>