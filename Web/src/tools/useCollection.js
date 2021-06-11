import { ref } from 'vue'
import { db } from '../firebase/config'

const useCollection = (collection) => {
    const error = ref(null)
    const isPending = ref(false)    


    const addDoc = async (doc) => {
        error.value = null
        isPending.value = true

        try {
            const res = await db.collection(collection).add(doc)
            isPending.value = false
            return res
        } catch (err) {
            console.log(err.message)            
            error.value = 'Could not send the message.'
            isPending.value = false
        }
    }


    const docInCollection = (meals, id) => {
        let res = meals.find((meal) => {
            return meal.mealId === id
        })
        // if (res) console.log('If true, RES = ',res)
        // if (!res) console.log('If false, RES = ',res)
        if (res) return true
        return false
    }

    return { error, addDoc, isPending, docInCollection}

}

export default useCollection