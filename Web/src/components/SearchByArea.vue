<template>
<div class="mainAreaPage" v-if="caiList">
        <div class="areaPage">
                <div class="categoryTitle">
                    <div id="luigiDiv">
                        <img id="luigi" src="@/assets/chefa.png">
                    </div>
                    <h1>World regions</h1>
                </div>
                <div class="areaMenu" v-if="caiList">
                    <div class="areaTitle"  v-for="area in caiList" :key="area.strArea">
                        <p @click="handleClickArea(area.strArea)" >{{area.strArea}}</p>
                    </div>
                </div>
            
            <MealList v-if="meals" :meals="meals"/>
        </div>
        <div id="globe">
            <img src="@/assets/globe.png">
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
        const res = await getCAIList('a')
    })

    const handleClickArea = async (area) => {
        const res = await getMealsByCAI('a', area)
    }

    return{ caiList, error, handleClickArea, meals }
}
}
</script>

<style scoped>
.mainAreaPage{
    display: flex;
    position: relative;
    margin: auto;
    justify-content: center;
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
    padding: 4px 15px;
    text-align: center;
    color: var(--button1);
    font-family: 'Great Vibes', cursive;
    font-size: 3em;
    border-radius: 50%;
    box-shadow:0 0 15px 10px rgba(255, 255, 255, 1.8);
    
    
    /* border-top:solid 2px var(--button1); */

}
.areaMenu{
    display: flex;
    flex-flow: row wrap;
    justify-content: space-between;
    margin-top:10px;
    margin-bottom: 40px;
    padding: 20px 0;
    background: white;
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
.mainAreaPage #globe{
    position: absolute;
    display: flex;
    top:0px;
    margin:0 auto;
    z-index: -4;
    align-items: center;
    justify-items: center;
    opacity: 0.3;

}
.mainAreaPage #globe img{
    height: 700px;
}

</style>