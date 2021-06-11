<template>
<div class="categoryPage">
    <div v-if="error" >{{ error }}</div>
    <div class="categoryTitle">
        <div id="luigiDiv">
          <img id="luigi" src="@/assets/chefa.png">
        </div>
        <h1>Food categories</h1>
    </div>
    <div v-if="categories.length" class="catMenu">
        
        <div v-for="cat in categories" :key="cat.idCategory">
            <h3 > <a class="catTitle" :href="'#c'+cat.strCategory">{{ cat.strCategory }} </a></h3>
        </div>
    </div>
    
    <div v-if="categories.length" class="catContainer">
        <div class="catSingle" v-for="cat in categories" :key="cat.idCategory">
            
                <h3 :id="'c'+cat.strCategory"> {{ cat.strCategory }}</h3>
                <div>
                    <img class="catImage" :src="cat.strCategoryThumb">
                    <p> {{ cat.strCategoryDescription }}</p>
                </div>
        </div>
    </div>
    <!-- <div v-if="!categories.length">Loading data...</div> -->
    <div v-if="!categories.length" class="lds-circle"><div></div></div>
</div>
</template>

<script>
import { ref } from 'vue'

export default {
  name: 'Categories',
  setup(){
    const categories = ref([])
    const error = ref(null)
    const uri = 'https://www.themealdb.com/api/json/v1/1/categories.php'
    const load = async () => {
      try {
        let data = await fetch(uri)
        if (!data.ok){
          throw Error('No data available')
        }
        categories.value = await data.json()
        categories.value = categories.value.categories
        
      } 
      catch (err) {
        error.value = err.message
        console.log(error.value)
      }
    }
  
  load()

    return { error, categories }
  }
  

}
</script>

<style scoped>
.categoryPage{
  margin-top: -30px;
}
.catContainer{
    display: grid;
    grid-template-columns: 1fr 1fr;
    grid-template-rows: auto auto;
}
.catContainer h3{
text-align: center;
font-family: 'Great Vibes', cursive;
color: var(--button1);
font-size: 3em;
margin-top: 20px;
}

.catContainer p{
text-align: justify;
margin: 15px;
}
.catContainer div{
padding: 20px;
text-align: center;

}
.catContainer .catSingle{
    background: rgb(254, 254, 254);
    border-radius: 0px;
    margin: 10px;
    border-bottom: solid 2px var(--button1);
}
.catContainer .catImage{
    margin: 10px;
    border-radius: 10px;
}
.catMenu{
    display: flex;
    flex-flow: row;
    flex-wrap: wrap;
    justify-content: space-between;
    margin:10px 10px;
    padding: 20px 0;
   
    border-radius: 0px;
    
    border-bottom: solid 2px var(--button1);
    border-top: solid 2px var(--button1);

    /* box-shadow: 0px 3px 4px rgba(50,50,50,0.4); */
}
h1{
    text-align: center;
    color: var(--button1);
    /* text-transform: uppercase; */
    font-family: 'Great Vibes', cursive;
    font-size: 3em;
}

.catTitle{
font-size: 0.8em;
font-weight: 400;
margin: 0px 10px 0px 10px;
transition: ease 0.4s;
padding: 4px 0px;
cursor: pointer;

}
.catTitle:hover{
    color: var(--button2);
    transition: ease 0.4s;
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
.categoryTitle{
  transform: translateY(32px);
  display: flex;
  flex-direction: column;
  align-items: center;
}
h1{
  background: white;
  padding: 0 15px;
  text-align: center;
  margin: 0;
    
}
</style>