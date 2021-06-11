<template>
    <div class="formContainer">
        <div>
            <form @submit.prevent="handleSubmit">
                <h3 class="formH3">Login</h3>
                <input type="email" placeholder="Email" v-model="email">
                <input :type="showPass" placeholder="Password" v-model="password">
                <button class="loginButton" >Log in</button>
                <div v-if="!isPending" class="logbuttons">
                    <div></div>
                    <span :class="{eye_active: showPass !== 'password'}" class="material-icons eye" @click="toggleShowPass">visibility</span>
                </div>
                
                <div v-if="errorLogin" class="error">{{ errorLogin }}</div>
                <!-- <div v-if="errorSign" class="error">{{ errorSign }}</div> -->
                <div v-if="isPending" class="lds-circle"><div></div></div> 
                
                <div class="googleSignup" @click="handleGoogleSignup">
                    <img class="googleLogo" src="@/assets/google.png" >
                    <p> Log in with Google</p>
                </div> 
                <div class="googleSignup" @click="handleFacebookSignup">
                    <img class="googleLogo" src="@/assets/facebook.png" >
                    <p> Sign up with Facebook</p>
                </div>
                <div class="signupLink">
                    <p>
                        No account yet? Go to <router-link :to="{name: 'Signup'}">Signup</router-link> form
                    </p>
                </div>
            </form>
        </div>
        <img id="chef2" src="@/assets/chefw.png" >
    </div>
    
</template>

<script>
import { ref } from 'vue'
import useLogin from '@/tools/useLogin.js'
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
     
    const {error: errorLogin, login, isPending} = useLogin()
    const {error:errorSign,  signupGoogle, signupFacebook} = useSignup()
    const email = ref('')
    const password = ref('')
    const router = useRouter()
    
    const handleSubmit = async () => {
        const res = await login(email.value, password.value)
        if(!errorLogin.value){
            router.push({name: 'Home'})
        }
    }

     const handleGoogleSignup = async () => {
        const res = await signupGoogle()
        if(!errorSign.value){
            router.push({name: 'Home'})       
        }
    }
    const handleFacebookSignup = async () => {
        const res = await signupFacebook()
        if(!errorSign.value){
            router.push({name: 'Home'})       
        }
    }

    return { showPass, toggleShowPass, email, password, handleSubmit, errorLogin, isPending, handleGoogleSignup, handleFacebookSignup, errorSign }
}
}

</script>

<style>
.logbuttons{
    display: flex;
    justify-content: space-between;
    align-items: center;
}
form .eye {
  color: var(--button1);  
  cursor: pointer;
  text-decoration: none;
  border-radius: 50%;
  padding: 5px;
  transform: translateY(-94px) translateX(-5px);

}
.formH3 {
    margin: 10px auto;
    text-transform: uppercase;
    text-align: center;
} 

.loginButton{
    margin: 0px;
    width: 100%;
}
form .eye_active{
    color:var(--button2);
}
.googleLogo{
    max-width:45px
}
.googleSignup{
    display: flex;
    flex-flow: row;
    justify-content: center;
    align-items: center;
    font-size: 14px;
    background: rgb(233, 233, 233);
    border-radius: 10px;
    padding:5px 10px;
    margin: 20px auto 0px auto;
    border-radius: 10px;
    transition: ease 0.5s;
    cursor: pointer;
}
.googleSignup:hover{
    transition: ease 0.5s;
    box-shadow:inset 0px 0px 6px rgba(50,50,50,0.5);
    background: white;
}
.formContainer{
    display: flex;
    flex-flow: row;
    justify-content: center;
    align-items: center;
}
#chef2{
    max-height: 240px;
    transform: translateX(-26px) translateY(70px) ;
}
.signupLink a{
    font-size: 1em;
    font-weight: 600;
    color: var(--button1);

}
.signupLink a:hover{
    font-size: 1em;
    font-weight: 600;
    color: var(--button2);

}
.signupLink{
    padding-top: 25px;
    text-align: center;
}
</style>