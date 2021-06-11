<template>
<div v-if="caiList">
    <div class="areaPage">
            <div class="categoryTitle">
                <div id="luigiDiv">
                    <img id="luigi" src="@/assets/chefa.png">
                </div>
                <h1>Food categories</h1>
            </div>
            <div class="areaMenu" v-if="caiList">
                  <div class="areaTitle"  v-for="category in caiList" :key="category.strCategory">
                      <p @click="handleClickArea(category.strCategory)" >{{category.strCategory}}</p>
                  </div>
            </div>
            <div class="categoryImages" v-if="caiList && !meals">
                <div>
                <img @click="handleClickArea('Beef')" src="https://www.themealdb.com/images/category/beef.png">
                <p>Beaf</p>
                </div>
                <div>
                <img @click="handleClickArea('Chicken')" src="https://www.themealdb.com/images/category/chicken.png">
                <p>Chicken</p>
                </div>
                <div>
                <img @click="handleClickArea('Dessert')" src="https://www.themealdb.com/images/category/dessert.png">
                <p>Dessert</p>
                </div>
                <div>
                <img @click="handleClickArea('Lamb')" src="https://www.themealdb.com/images/category/lamb.png">
                <p>Lamb</p>
                </div>

                <div>
                <img @click="handleClickArea('Miscellaneous')" src="https://www.themealdb.com/images/category/miscellaneous.png">
                <p>Miscellaneous</p>
                </div>
                <div>
                <img @click="handleClickArea('Pasta')" src="https://www.themealdb.com/images/category/pasta.png">
                <p>Pasta</p>
                </div>
                <div>
                <img @click="handleClickArea('Pork')" src="https://www.themealdb.com/images/category/pork.png">
                <p>Pork</p>
                </div>
                <div>
                <img @click="handleClickArea('Seafood')" src="https://www.themealdb.com/images/category/seafood.png">
                <p>Seafood</p>
                </div>
                <div>
                <img @click="handleClickArea('Side')" src="https://www.themealdb.com/images/category/side.png">
                <p>Side</p>
                </div>
                <div>
                <img @click="handleClickArea('Starter')" src="https://www.themealdb.com/images/category/starter.png">
                <p>Starter</p>
                </div>
                <div>
                <img @click="handleClickArea('Vegan')" src="https://www.themealdb.com/images/category/vegan.png">
                <p>Vegan</p>
                </div>
                <div>
                <img @click="handleClickArea('Vegetarian')" src="https://www.themealdb.com/images/category/vegetarian.png">
                <p>Vegetarian</p>
                </div>
                <div>
                <img @click="handleClickArea('Breakfast')" src="https://www.themealdb.com/images/category/breakfast.png">
                <p>Breakfast</p>
                </div>
                <div>
                <img @click="handleClickArea('Goat')" src="https://www.themealdb.com/images/category/goat.png">
                <p>Goat</p>
                </div>
            </div>
            <MealList v-if="meals" :meals="meals"/>
      
          

    </div>
</div>    
<div v-if="!caiList" class="lds-circle"><div></div></div>
</template>

<script>
import MealList from './MealList.vue'
import getMeal from '@/tools/getMeal.js'
import { onMounted } from 'vue'
export default {
components:{ MealList },
setup(){
    const {caiList, error, getCAIList, meals, getMealsByCAI} = getMeal()

    onMounted(async ()=> {
        const res = await getCAIList('c')
    })

    const handleClickArea = async (category) => {
        const res = await getMealsByCAI('c',category)
    }

    return{ caiList, error, handleClickArea, meals }
}
}
</script>

<style scoped>
.categoryImages{
  display: flex;
  flex-flow: row wrap;
  padding: 20px auto;
  margin: auto;
  justify-content: space-around;
  margin-bottom: 100px;
}
.categoryImages p{
  text-align: center;
  margin: 0;
  font-family: 'Great Vibes', cursive;
  font-size: 2em;
  font-weight: 400;

}
.categoryImages img{
  max-height: 120px;
  margin: 15px 7px;
  padding: 0px 10px;
  cursor: pointer;
  border-radius: 50%;
  transition: ease 0.5;

}
.categoryImages img:hover{
  transition: ease 0.5;
  transform: scale(1.1);

}
.areaPage{
     margin-top: -30px;
}
.areaTitle{
    font-size: 0.90em;
    font-weight: 400;
    margin: 0px 10px 0px 10px;
    transition: ease 0.4s;
    padding: 4px 4px;
    cursor: pointer;
}
#luigiDiv{
  text-align: center;
  display: flex;
  justify-content: center;
}
#luigi{
  max-height: 100px;
  max-width: 100px;
  margin:0;
  transform: translateY(20px);
  z-index: 1;
 
}
h1{
    background: white;
    padding: 0 15px;
    text-align: center;
    color: var(--button1);
    font-family: 'Great Vibes', cursive;
    font-size: 3em;
}
.areaMenu{
    display: flex;
    flex-flow: row wrap;
    justify-content: space-between;
    margin-top:10px;
    margin-bottom: 40px;
    padding: 20px 0;
   
    border-radius: 0px;
    
    border-bottom: solid 2px var(--button1);
    border-top: solid 2px var(--button1);

    /* box-shadow: 0px 3px 4px rgba(50,50,50,0.4); */
}
.categoryTitle{
  transform: translateY(30px);
  display: flex;
  flex-direction: column;
  align-items: center;
}


.mealListItem{
  display: grid;
  grid-template-columns: 1fr  3fr 1fr 1fr 1fr;
  justify-self: start;
  align-self: center;
  align-items: center;
  column-gap: 15px;
  padding: 20px;
  border-radius: 100px;
  margin:15px 100px ;
  /* border-top: solid 1px; */
  border-bottom: solid 1px;
  transition: ease 0.3s;
  box-shadow: 2px 2px 3px rgba(50,50,50,0.5); 
  background: rgb(245, 245, 245);
  }
  .testing:hover{
    transition: ease 0.3s;
    transform: scale(1.02);
    background: rgb(240, 240, 240);
    
  }
.image{
    margin-right:  auto;
    max-width: 250px;
    max-height: 130px;
    object-fit: cover;
    border-radius: 50%;
    /* border: solid 5px white; */
    box-shadow: 2px 2px 3px rgba(50,50,50,0.5); 
}
#titleMeal{
    font-family: 'Great Vibes', cursive; 
    font-size: 2em;
}
</style>