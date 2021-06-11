<template>
<div v-if="favorites">
  <div class="favoritesList" v-for="favMeal in favorites" :key="favMeal.mealId">
     <MealBar v-if="favMeal" :passedMealId="favMeal.mealId" :favorites="favorites" :favId="favMeal.id" />
      
  </div>
</div>
</template>

<script>
import MealBar from './MealBar.vue'
import useDocument from '@/tools/useDocument.js'
export default {
  components: { MealBar },
  props: ['favorites'],


setup(){
  const { error, deleteDoc, isPending } = useDocument('favorites')

  return {}
}
}
</script>

<style scoped>
.favoritesList{
  text-align: center;
}
</style>