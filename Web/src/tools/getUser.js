import { ref } from "@vue/reactivity"
import { dbAuth } from "../firebase/config"

const user = ref(dbAuth.currentUser)

dbAuth.onAuthStateChanged((_user) => {
    // console.log('User state changed. Current user is: ',_user)
    user.value = _user
})

const getUser = () => {
    return { user }
}

export default getUser