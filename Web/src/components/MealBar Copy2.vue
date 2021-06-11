<template>
<div class="error" v-if="error">{{error}}</div>
<div class="entireBar" v-if="meal">
  <router-link :to="{name:'MealDetails', params: {id: meal.idMeal} }">
              
      <div class="mealListItem" >
                  <img class="image" :src="meal.strMealThumb">
                    <div>
                        <p :class="{titleMeal: !titleFontFlag, titleMealSmall: titleFontFlag}">{{meal.strMeal}}</p>
                        <p>{{meal.strArea}} {{meal.strCategory}}</p>
                    </div>
                    <div class="ingredientsList">
                        <h4 id="ingrHeader"><span class="material-icons">lunch_dining</span> Main ingredients:</h4>
                        <ul>
                                <li v-if="meal.strIngredient1">{{meal.strIngredient1}}</li>
                                <li v-if="meal.strIngredient2">{{meal.strIngredient2}}</li>
                                <li v-if="meal.strIngredient3">{{meal.strIngredient3}}</li>
                        </ul>
                        <ul >
                                <li v-if="meal.strIngredient5">{{meal.strIngredient5}}</li>
                                <li v-if="meal.strIngredient6">{{meal.strIngredient6}}</li>
                                <li v-if="meal.strIngredient7">{{meal.strIngredient7}}</li>
                        </ul>
                    </div>
                    <!-- <h3>{{ meal.idMeal }}</h3> -->
      </div>
  </router-link>
  <div class="boardButton">
    <img class="board" src="@/assets/board.png">
    <div v-if="favorites" class="favoriteButtons">
      <button @click="handleRemoveEvent" class="delBtn">Remove</button>
    </div>
    
  </div>
  <div class="error" v-if="errorFirebase">{{errorFirebase}}</div>
</div>
</template>

<script>
import getMeal from '@/tools/getMeal.js'
import { onBeforeMount, onMounted, ref } from 'vue'
import useDocument from '@/tools/useDocument.js'
export default {
    props: ['passedMealId', 'favorites', 'favId'],
    setup(props){
        const {meal, error, isPending, getMealById } = getMeal()
        const {error: errorFirebase, isPending: isPendingFirebase, deleteDoc } = useDocument('favorites',props.favId )
        let titleFontFlag = ref(false)
        
        onBeforeMount( async ()=>{
            await getMealById(props.passedMealId)
            await console.log(meal.value, ' ',meal.value.strMeal.length)
            if(meal.value.strMeal.length > 32){
                titleFontFlag.value = true
            }
                await console.log(titleFontFlag.value)
        })
       const handleRemoveEvent = async ()=>{
           const res = await deleteDoc()
           removeComment.value = 'Meal: '+ meal.value.strMeal + ' was removed from favorites.'
           console.log(removeComment.value)
       }

        return {meal, error, errorFirebase, isPending, titleFontFlag, handleRemoveEvent}
    }

}
</script>

<style>
.entireBar{
  display: flex;
  justify-items: center;
  align-items: center;
}
.board{
  
  max-height: 90px;
  transform: translateX(-50px);
}
.boardButton{
  position: relative;
}
.mealListItem{
  display: grid;
  grid-template-columns: 1fr 5fr 4fr auto;
  justify-self: start;
  justify-items: start;
  align-self: center;
  align-items: center;
  column-gap: 20px;
  padding: auto;
  border-radius: 100px;
  margin:15px auto ;
  max-width: 900px;
  border-bottom: solid 1px;
  transition: ease 0.3s;
  box-shadow: 2px 2px 3px rgba(50,50,50,0.5); 
  background: rgb(248, 248, 248);
  text-align: left;
  z-index: 1;
  }

  .mealListItem:hover{
    transition: ease 0.3s;
    transform: scale(1.02);
    background: rgb(235, 235, 235);
    cursor: pointer;
    
  }
.mealListItem .image{
    margin-right: auto;
    max-width: 250px;
    max-height: 120px;
    object-fit: cover;
    border-radius: 50%;
    /* border: solid 5px white; */
    box-shadow: 2px 2px 3px rgba(50,50,50,0.5); 
}
.mealListItem .titleMeal{
    font-family: 'Great Vibes', cursive; 
    font-size: 2em;
}
.mealListItem .titleMealSmall{
    font-family: 'Great Vibes', cursive; 
    font-size: 1.6em;
}
.mealListItem .ingredientsList{
    align-self: center;
    justify-self: start;
    /* margin-right:60px; */
    display: grid;
    grid-template-columns: auto auto;
    column-gap: 24px;
    font-size: 0.7em;
    /* border-left: solid 1px #888;  */
    padding-left: 10px ;
}
.mealListItem #ingrHeader{
    /* border-bottom: solid 1px; */
    font-size: 1em;
    font-weight: 600;
    grid-column: 1 / 3;
    margin-bottom: 4px;
}
.mealListItem .material-icons{
    color: var(--primary);
    vertical-align: middle;
    font-size: 1.8em;
}
.mealListItem li{
    margin-left:18px;
}
.favoriteButtons{

  position: absolute;
  top: 6px;
  left: -80px;
  /* display: flex; */
  /* flex-flow: column wrap; */
  /* justify-content: center; */
  /* overflow: hidden; */
  /* padding: 15px 15px; */
  margin:0;
}
.favoriteButtons button{
  color: white; 
  padding: 24px 8px;
  border-radius: 50%;
  font-weight: 400;
}

.delBtn{
  background: var(--button1);
  box-shadow: inset 14px 0px 10px rgb(0, 121, 177);
  z-index: 2;

}
.delBtn:hover{
  background: var(--button2);
  box-shadow: inset 14px 0px 10px rgb(177, 121, 0);
}
</style>