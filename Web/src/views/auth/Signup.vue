<template>
<div class="formContainer">
    <div>
        <form @submit.prevent="handleSubmit">
            <h3 class="formH3">Sign up</h3>

            <div>
                <img :src="photo">
            </div>

            <input type="text" placeholder="Display name" v-model="displayName">
            <input type="email" placeholder="Email" v-model="email">
            <input :type="showPass" placeholder="Password" v-model="password">
            
            <button class="loginButton">Sign up</button>
            <div v-if="!isPending" class="logbuttons">
                <div></div>
                <span :class="{eye_active: showPass !== 'password'}" class="material-icons eye" @click="toggleShowPass">visibility</span>
            </div>
            <div v-if="errorSignup" class="error">{{ errorSignup }}</div>
            <div class="googleSignup" @click="handleGoogleSignup">
                <img class="googleLogo" src="@/assets/google.png" >
                <p> Sign up with Google</p>
            </div>
            <div class="googleSignup" @click="handleFacebookSignup">
                <img class="googleLogo" src="@/assets/facebook.png" >
                <p> Sign up with Facebook</p>
            </div>
            <div v-if="isPending" class="lds-circle"><div></div></div> 
             <div class="signupLink">
                    <p>
                        Allready got the account. Go to <router-link :to="{name: 'Login'}">Login</router-link> form
                    </p>
                </div>
        </form>
  </div>
  <img id="chef2" src="@/assets/chefw.png" >
</div>
</template>

<script>
import { onUpdated, ref } from 'vue'
import useSignup from '@/tools/useSignup.js'
import { useRouter } from 'vue-router'
export default {
setup(){

    // toggle show password
    const showPass = ref('password')
    const toggleShowPass = () => {
        if(showPass.value === "password") 
        { showPass.value = 'text' } else { showPass.value = "password" }
    } 


    const email = ref('')
    const password = ref('')
    const displayName = ref('')
    const router = useRouter()
    const photo = ref('')
    
    const { error: errorSignup, signup, signupGoogle, signupFacebook, isPending } = useSignup()
    

    const handleSubmit = async () => {
       const res = await signup(email.value, password.value, displayName.value)
       if(!errorSignup.value){
            router.push({name: 'Home'})       }
    }
    const handleGoogleSignup = async () => {
        const res = await signupGoogle()
        if(!errorSignup.value){
            photo.value = await res.user.photoURL
            await router.push({name: 'Home'})       
        }
    }
    const handleFacebookSignup = async () => {
        const res = await signupFacebook()
        if(!errorSignup.value){
            router.push({name: 'Home'})       
        }
    }
  
    return { email, password, displayName, handleSubmit, errorSignup, isPending, showPass, toggleShowPass, handleGoogleSignup, handleFacebookSignup, photo }
}
}
</script>

<style>

</style>