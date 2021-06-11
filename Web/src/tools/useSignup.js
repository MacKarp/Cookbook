import { ref } from "@vue/reactivity"
import { dbAuth } from "../firebase/config"
import firebase from "firebase"


const error = ref(null)
const isPending = ref(false)   

const signup = async (email, password, displayName) => {
    error.value = null
    isPending.value = true

    try{
        const res = await dbAuth.createUserWithEmailAndPassword(email, password)
        if (!res){
            throw new Error('Could not complete the signup.')
        }
        await res.user.updateProfile({displayName})
        error.value = null
        isPending.value = false
        return res

    } catch(err) {
        console.log(err.message)
        error.value = err.message
        isPending.value = false
    }
}

const signupGoogle = async () =>{
    error.value = null
    isPending.value = true
    const googleProvider = new firebase.auth.GoogleAuthProvider() 
    
    try{
        const res = await dbAuth.signInWithPopup(googleProvider)
        if (!res){
            throw new Error('Could not complete the google signup.')
        }
        error.value = null
        isPending.value = false
        return res

    } catch(err) {
        console.log(err.message)
        error.value = err.message
        isPending.value = false
    }

}

const signupFacebook = async () =>{
    error.value = null
    isPending.value = true
    const facebookProvider = new firebase.auth.FacebookAuthProvider()
    
    try{
        const res = await dbAuth.signInWithPopup(facebookProvider)
        if (!res){
            throw new Error('Could not complete the Facebook signup.')
        }
        error.value = null
        isPending.value = false
        return res

    } catch(err) {
        console.log(err.message)
        error.value = err.message
        isPending.value = false
    }

}

const useSignup = () => {

    return { error, signup, signupGoogle, signupFacebook, isPending}
}

export default useSignup