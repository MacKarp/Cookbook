<template>
  <div>
        
            <div id="luigiDiv">
           
            <div class="headerFavorites">
                <h2>Hello Chef!</h2>
                <h4>Here's your favorites:</h4>
            </div>
                <img id="luigi" src="@/assets/chefa.png">
            </div> 
            <div v-if="favorites">
                
                <FavoritesList :favorites="favorites" />
            </div>
       
  </div>
</template>

<script>
import getCollection from '@/tools/getCollection.js'
import getUser from '@/tools/getUser.js'
import FavoritesList from '@/components/FavoritesList.vue'
export default {
    components: { FavoritesList },
    setup(){
        const { user } = getUser()
        const { documents: favorites} = getCollection(
            'favorites',
            ['userId', '==', user.value.uid ] )

               
        
        return { favorites, user }
    }

}


</script>

<style scoped>
h2{
    font-family: 'Great Vibes', cursive;
    font-weight: 300;
    font-size: 2.5em;
    border-bottom: solid 1px;
    
}
h4{
    font-weight: 400;
    
}

.headerFavorites{
    text-align: center;
    margin: 30px 0;
}
#luigiDiv{
  text-align: center;
  display: flex;
  flex-flow: row wrap;
  align-items: flex-start;
  justify-content: center;
}
#luigi{
  max-height: 100px;
  max-width: 100px;
  margin:0;
  transform: translateY(20px);
  z-index: 1;
 
}

</style>