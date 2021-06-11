import { ref } from 'vue'

const getMeal = () => {

const isPending = ref(false)
const caiList = ref(null)
const meal = ref(null)
const meals = ref(null)
const error = ref(null)
const uriById = 'https://www.themealdb.com/api/json/v1/1/lookup.php?i='
const randomUri = 'https://www.themealdb.com/api/json/v1/1/random.php'
const uriByName = 'https://www.themealdb.com/api/json/v1/1/search.php?s='
const uriCAI = 'https://www.themealdb.com/api/json/v1/1/list.php?' //c=list'
const uriFilterCAI = 'https://www.themealdb.com/api/json/v1/1/filter.php?' //a=

const getCAIList = async (cai) => {
  try {
    isPending.value = true
    let data = await fetch(uriCAI + cai + '=list')
    if (!data.ok){
      throw Error('No data available')
      isPending.value = false
    }
    caiList.value = await data.json()
    caiList.value = caiList.value.meals
    isPending.value = false
  } 
  catch (err) {
    error.value = err.message
    console.log(error.value)
    isPending.value = false
  }
}

const getMealsByCAI = async (caiType, caiValue) => {
  try {
    isPending.value = true
    let data = await fetch(uriFilterCAI + caiType + '=' + caiValue)
    if (!data.ok){
      throw Error('No data available')
     
    }
    meals.value = await data.json()
    meals.value = meals.value.meals
    isPending.value = false
  } 
  catch (err) {
    error.value = err.message
    console.log(error.value)
    isPending.value = false
  }
}



const getMealById = async (id) => {
      try {
        isPending.value = true
        let data = await fetch(uriById + id)
        if (!data.ok){
          throw Error('No data available')
        }
        meal.value = await data.json()
        // = meal.value.meals[0]
        meal.value = meal.value.meals[0]
        isPending.value = false
      } 
      catch (err) {
        error.value = err.message
        console.log(error.value)
        isPending.value = false
      }
    }

    const getMealsByName = async (mealName) => {
      try {
        isPending.value = true
        let data = await fetch(uriByName + mealName)
        if (!data.ok){
          throw Error('No data available')
        }
        meals.value = await data.json()
        meals.value = meals.value.meals
        isPending.value = false
      } 
      catch (err) {
        error.value = err.message
        console.log(error.value)
        isPending.value = false
      }
    }

    const getRandomMeal = async () => {
        try {
          isPending.value = true
          let data = await fetch(randomUri)
          if (!data.ok){
            throw Error('No data available')
          }
          meal.value = await data.json()
          // = meal.value.meals[0]
          meal.value = meal.value.meals[0]
          isPending.value = false
        } 
        catch (err) {
          error.value = err.message
          console.log(error.value)
          isPending.value = false
        }
      }
    return {meal, meals, error, isPending, getMealById, getRandomMeal, getMealsByName, getCAIList, getMealsByCAI, caiList}
}
    export default getMeal