import { ref } from "@vue/reactivity"
import { dbAuth } from "../firebase/config"

const error = ref(null)
const isPending = ref(false)    

const logout = async () => {
    error.value = null
    isPending.value = true
    try {
        await dbAuth.signOut()
        isPending.value = false
    } catch (err) {
        console.log(err.message)
        error.value = err.message
        isPending.value = false
    }
}

const useLogout = () => {
    return { logout, error, isPending}
}

export default useLogout