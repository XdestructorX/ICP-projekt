<script setup>
import { ref } from 'vue';
import { praktyka_backend } from 'declarations/praktyka_backend/index';

let input = ref("");
const fruits = ["apple", "banana", "orange"];
function filteredList() {
  return fruits.filter((fruit) =>
    fruit.toLowerCase().includes(input.value.toLowerCase())
  );
}
</script>

<template>
  <main>
   <input type="text" class="text-lg" v-model="input" placeholder="Search fruits..." />
   <div class="item fruit" v-for="fruit in filteredList()" :key="fruit">
     <p>{{ fruit }}</p>
    </div>
    <div class="item error " v-if="input&&!filteredList().length">
      <p>No results found!</p>
    </div>
  </main>
</template>
