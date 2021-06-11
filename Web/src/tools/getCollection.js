import { ref, watchEffect } from 'vue'
import { db } from '../firebase/config'


const getCollection = (collection, query) => {
    const documents = ref(null)
    const error = ref(null)

    
    let collectionRef = db.collection(collection)
        .orderBy('createdAt')

    if (query){
        collectionRef = collectionRef
            .where(...query)
    }
        
    const unsub = collectionRef.onSnapshot((snap) => {
        let results = []
        snap.docs.forEach(doc => {
            doc.data().createdAt && results.push({ ...doc.data(), id: doc.id})
        })
        documents.value = results
        error.value = null
    }, (err)=>{
        console.log(err.message)
        documents.value = null
        error.value = 'could not fetch data'
    })

    watchEffect((onInvalidate) =>{
        onInvalidate(() => unsub())
    })
    
    return { error, documents }
}

export default getCollection