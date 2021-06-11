<template>
    <!-- v-if="favorites.length"  -->
  <button 
            :id="buttonId" 
            :class="{buttonDisabled : mealButtonFlag}" 
            @loadeddata="handleEvent()"
            @click="handleAddToFavotites(meal, buttonId)"
            ><span class="material-icons">favorite</span>Add to your favorites
    </button>
</template>

<script>
import getUser from '@/tools/getUser.js'
import useCollection from '@/tools/useCollection'
import { timestamp } from '@/firebase/config.js'
import { ref, watch, watchEffect } from 'vue'

export default {
    props: ['meal','favorites','buttonId'],
    setup(props){
    const { user } = getUser()    
    const { error, addDoc, isPending, docInCollection} = useCollection('favorites')
    const mealButtonFlag = ref(false)
    
    
    const handleEvent = async () =>{
          mealButtonFlag.value = await  docInCollection(props.favorites, props.meal.idMeal)
          if (mealButtonFlag.value) {
              document.getElementById(props.buttonId).innerHTML = '<span class="material-icons">favorite</span>Your favorite'
              document.getElementById(props.buttonId).disabled = true
          }
      }

    watchEffect(() => {
        handleEvent()
    }) 

    const handleAddToFavotites = async (recipe, buttonId) => {
        
            isPending.value = true
            console.log(recipe)
            const res = await addDoc({
                 mealId: recipe.idMeal,
                 mealName: recipe.strMeal,
                 mealArea: recipe.strArea,
                 mealCategory: recipe.strCategory,
                 mealPhotoUrl: recipe.strMealThumb,
                 userId: user.value.uid,
                 userName: user.value.displayName,

                 createdAt: timestamp()
                 })
        
         if(!error.value) {
                console.log('Meal with id: ',recipe.idMeal, ' was added to database.')
                mealButtonFlag.value = true
                document.getElementById(buttonId).innerHTML = "Added to favorites"
                document.getElementById(buttonId).disabled = true
                isPending.value = false
         }
      }
    return { handleEvent, docInCollection, handleAddToFavotites, error, isPending, mealButtonFlag}
}
}
</script>

<style scoped>
button {
    display: flex;
    justify-items: center;
    align-items: center;
    width: 100%;
}

</style>