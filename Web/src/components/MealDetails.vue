<template>
<div v-if="meal">
   <div v-if="meal" class="mealContainer">
     <div class="mealSubContainer">
        <h2 class="recipeTitle">{{meal.strMeal}}</h2>
        <div class="ingrContainer">
          <div>
            <h4 >INGREDIENTS:</h4>
            <ul class='list'>
                
                <li v-if="meal.strIngredient1">{{meal.strIngredient1}}: {{meal.strMeasure1}}</li>
                <li v-if="meal.strIngredient2">{{meal.strIngredient2}}: {{meal.strMeasure2}}</li>
                <li v-if="meal.strIngredient3">{{meal.strIngredient3}}: {{meal.strMeasure3}}</li>
                <li v-if="meal.strIngredient4">{{meal.strIngredient4}}: {{meal.strMeasure4}}</li>
                <li v-if="meal.strIngredient5">{{meal.strIngredient5}}: {{meal.strMeasure5}}</li>
                <li v-if="meal.strIngredient6">{{meal.strIngredient6}}: {{meal.strMeasure6}}</li>
                <li v-if="meal.strIngredient7">{{meal.strIngredient7}}: {{meal.strMeasure7}}</li>
                <li v-if="meal.strIngredient8">{{meal.strIngredient8}}: {{meal.strMeasure8}}</li>
                <li v-if="meal.strIngredient9">{{meal.strIngredient9}}: {{meal.strMeasure9}}</li>
                <li v-if="meal.strIngredient10">{{meal.strIngredient10}}: {{meal.strMeasure10}}</li>
                <li v-if="meal.strIngredient11">{{meal.strIngredient11}}: {{meal.strMeasure11}}</li>
                <li v-if="meal.strIngredient12">{{meal.strIngredient12}}: {{meal.strMeasure12}}</li>
                <li v-if="meal.strIngredient13">{{meal.strIngredient13}}: {{meal.strMeasure13}}</li>
                <li v-if="meal.strIngredient14">{{meal.strIngredient14}}: {{meal.strMeasure14}}</li>
                <li v-if="meal.strIngredient15">{{meal.strIngredient15}}: {{meal.strMeasure15}}</li>
                <li v-if="meal.strIngredient16">{{meal.strIngredient16}}: {{meal.strMeasure16}}</li>
                <li v-if="meal.strIngredient17">{{meal.strIngredient17}}: {{meal.strMeasure17}}</li>
                <li v-if="meal.strIngredient18">{{meal.strIngredient18}}: {{meal.strMeasure18}}</li>
                <li v-if="meal.strIngredient19">{{meal.strIngredient19}}: {{meal.strMeasure19}}</li>
                <li v-if="meal.strIngredient20">{{meal.strIngredient20}}: {{meal.strMeasure20}}</li>
            </ul>  
          </div>
        </div>
        <div class="categoryContainer">
            
            <AddToFavorites v-if="favorites && favorites" :buttonId="'favButton'" :meal="meal" :favorites="favorites" />  
            <div>
              <h4>CATEGORY:</h4>
              <p>{{meal.strCategory}}</p>
              <h4>AREA:</h4>
              <p>{{meal.strArea}}</p>
            </div>  
          </div>
      </div>
      <div >
        <img :src="meal.strMealThumb" class="mealImg">
      </div>
      
  </div>
  <h3 v-if="meal" >{{meal.strMeal}} </h3>
  <div :class="{instructionsContainer: videoFlag, instructionsContainerNoMovie: !videoFlag}">
    <div class="meal">
          <p>Recipe instructions:</p>
          <p class="para">
            <img class="coock" src="@/assets/chefa.png">
            <span v-if="meal">
              {{meal.strInstructions}}
            </span>
          </p>
    </div>
          <!-- :src="meal.strYoutube"  width="560" height="315"-->
    <div v-if="videoFlag && videoUrl" class="videoContainer">
        <iframe  
          :src="videoUrl"
          title="YouTube video player" 
          frameborder="0" 
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture" 
          allowfullscreen
          ></iframe>

    </div>


  </div>
</div>
<div v-if="!meal" class="lds-circle"><div></div></div>
</template>

<script>
import getCollection from '@/tools/getCollection.js'
import getMeal from '@/tools/getMeal.js'
import getUser from '@/tools/getUser.js'
import AddToFavorites from '@/components/AddToFavorites.vue'
import { onBeforeMount, ref } from 'vue'
export default {
  name: 'MealDetails',
  components: { AddToFavorites },
  props: ['id'],
  setup(props){
    const {meal, error, isPending, getMealById } = getMeal()
    const { user } = getUser()
    const videoFlag = ref(false)
    const videoUrl = ref(null)
        const { documents: favorites} = getCollection(
            'favorites',
            ['userId', '==', user.value.uid ] )


    onBeforeMount( async ()=>{
            const res = await getMealById(props.id)
            
            if(meal.value.strYoutube.length > 1){
              console.log(meal.value.strYoutube.toString())
              videoUrl.value = meal.value.strYoutube.replace('https://www.youtube.com/watch?v=', 'https://www.youtube.com/embed/')
              console.log(videoUrl.value.toString())
              videoFlag.value = true
            }
        })

    return { meal, error, isPending, favorites, videoFlag, videoUrl }
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
.para{
    margin:30px 0;
}
h3{
      font-family: 'Great Vibes', cursive;
      font-size: 2.5em;
      font-weight: 400;
}
h3{
  margin:15px auto;
  border-bottom: solid 1px;
  text-align: right;
}
iframe{
  margin-top: 40px;
 
   width: 100%;
   min-height: 400px;
   
   border-radius: 10px;
   box-shadow: 2px 2px 4px rgba(50,50,50,0.5);
}

</style>