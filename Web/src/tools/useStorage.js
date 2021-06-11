import { ref } from "@vue/reactivity"
import { db, dbStorage } from "../firebase/config"
import getUser from './getUser.js'

const { user } = getUser()

const useStorage= () => {
    const error = ref(null)
    const url = ref(null)
    const filePath = ref(null)

    const uploadImage = async (file) =>{
        filePath.value = `covers/${user.value.uid}/${file.name}`
        const storageRef = dbStorage.ref(filePath.value)

        try {
            const res = await storageRef.put(file)
            url.value = await res.ref.getDownloadURL()
        } catch (err) {
            console.log(err.message)    
            error.value = err.message
        }
    }

    const deleteImage = async (path) => {
        const storageRef = dbStorage.ref(path)

        try {
            await storageRef.delete()
        } catch (err) {
            console.log(err.message)
            error.value = err.message
        }
    }

    return { url, filePath, error, uploadImage, deleteImage }
}

export default useStorage