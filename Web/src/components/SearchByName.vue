<template>
    <form class="searchForm" @submit.prevent="handleSubmit">
        
        <label>Search by meal name:</label>
        <input type="text" placeholder="type in a meal name..." v-model="mealName">
        
        <button id="searchButton">Search</button>
    </form>
    <div>
        <div v-if="isPending">
             <div class="lds-circle"><div></div></div> 
        </div>
        <div v-else>
            <MealList v-if="meals" :meals="meals"/>
        </div>
    
    <div id="noResult" v-if="resultFlag">
       Sorry Chef... no such meal here
    </div>
    </div>
</template>

<script>
import MealList from './MealList.vue'
import { ref } from 'vue'
import getMeal from '@/tools/getMeal.js'
const {meals, error, isPending, getMealsByName} = getMeal()
export default {
    components:{ MealList },
    setup(){
        const mealName = ref(null)
        let resultFlag = ref(false)
        const handleSubmit = async () =>{
            const res = await getMealsByName(mealName.value)
            console.log(meals.value)
            console.log(resultFlag.value)
            resultFlag.value = false
            if (meals.value == null && mealName.value != null){
                resultFlag.value = true
            }
        }
        return {isPending, mealName, handleSubmit, getMealsByName, meals, error, resultFlag}
    }
}
</script>

<style scoped>
.testing{
  display: grid;
  grid-template-columns: 1fr 6fr 2fr 2fr 1fr;
  justify-self: start;
  align-self: center;
  align-items: center;
  column-gap: 15px;
  padding: 8px;
  background: rgb(246, 246, 246);
    border-radius: 10px;
    margin-bottom: 15px ;
    
  transition: ease 0.5s;
  }
  .testing:hover{
    transition: ease 0.5s;
    transform: scale(1.02);
    box-shadow: 2px 2px 4px rgba(50,50,50,0.5);
  }
.image{
    margin-right: 20px;
    max-width: 240px;
    max-height: 140px;
    object-fit: cover;
    border-radius: 10px;
    /* border: solid 5px white; */
    /* box-shadow: 2px 2px 3px rgba(50,50,50,0.5);  */
}
.favoriteButtons{
  display: flex;
  flex-flow: column wrap;
  justify-content: center;
}
.delBtn{
  background: var(--button2);
}
.delBtn:hover{
  background: var(--warning);
}
#titleMeal{
    font-family: 'Great Vibes', cursive; 
    font-size: 2em;
}
.searchForm{
    max-width: 500px;
    background:none;
    border: none;
    box-shadow: none;
    margin: 20px auto;
    display: grid;
    grid-template-columns: 6fr 1fr ;
    grid-template-rows:24px 40px;
    column-gap: 20px;
    row-gap: 15px;
    /* justify-self: center; */
    padding: 10px 20px ;

}
.searchForm #searchButton{
    place-self:center;
    /* margin-bottom: 0; */
}
.searchForm #searchBar{
    place-self: center;
}
.searchForm input{
    place-self: center;
    margin-bottom: 0;
    margin-top:0;
}
.searchForm label{
   grid-column-start: 1;
   grid-column-end: 3;
}
#noResult{
    font-family: 'Great Vibes', cursive; 
    font-size: 2em;
    text-align: center;
    margin: 40px auto;
}
</style>