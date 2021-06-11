import  { ref } from 'vue'
import { db } from '../firebase/config'

const useDocument = (collection, id) => {
    const error = ref(null)
    const isPending = ref(false)

    let docRef = db.collection(collection).doc(id)

    const deleteDoc = async () => {
        isPending.value = true
        error.value = null

        try {
            const res = await docRef.delete()
            isPending.value = false
            return res            
        } catch (err) {
            console.log(err.message)
            console.log(id)
            error.value = err.message
            isPending.value = false
        }
    }

    const updateDoc = async (updates) => {
        isPending.value = true
        error.value = null
        try {
            const res = await docRef.update(updates)
            isPending.value = false
            return res   
            
        } catch (err) {
            console.log(err.message)
            error.value = err.message
            isPending.value = false
        }
    }

    
    return {error, isPending, deleteDoc, updateDoc}
}

export default useDocument