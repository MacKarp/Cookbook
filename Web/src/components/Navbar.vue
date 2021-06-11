<template>
    <div class="navbar">
        <nav>
            <router-link :to="{name:'Home'}"><img src="@/assets/logo.png" ></router-link>
            <div class="navHeader">
                <h1><router-link :to="{name:'Home'}">API.<span class="blue">Cookbook</span> </router-link></h1>
                <h5>PWSZ N3PAM ZMP project</h5>
            </div>
            <div class="links">
                <div v-if="user">
                    <router-link :to="{name: 'UserList' }" class="play"> Favorites </router-link>      
                </div>
                <!-- <div><span id="userNameDiv"></span></div> -->
                <div v-if="user">
                    <span v-if="user.displayName" id="userNameDiv">Welcome {{user.displayName}}</span>
                    <span v-else id="userNameDiv">Welcome {{user.email}}</span>
                </div>
                <div v-if="user">
                    <div v-if="user.photoURL !== null" >
                        <img :src="user.photoURL" class="userphoto" @click="handleClickTest">
                    </div>
                    <div v-else>
                        <img src="@/assets/chefa.png" @click="handleClickTest">
                    </div>
                </div>
                <div v-if="user">
                    <button @click="handleClick">Logout</button>
                </div>


                <div v-else>
                    <router-link :to="{ name: 'Signup' }" class="btn">Sign up</router-link>
                    <router-link :to="{ name: 'Login' }" class="btn">Log in</router-link>
                </div>

            </div>
        
        </nav>
        <div v-if="user" class="links2">
            <router-link class="btnNav" :to="{name: 'Categories'}">Food Categories</router-link>
            <router-link class="btnNav" :to="{name: 'Home'}">Random recipes</router-link>
            <router-link class="btnNav" :to="{name: 'SearchByCategory'}">Search by category</router-link>
            <router-link class="btnNav" :to="{name: 'SearchByArea'}">Search by world region</router-link>
            <router-link class="btnNav" :to="{name: 'SearchByName'}">Search by name</router-link>
            <router-link class="btnNav" :to="{name: 'SearchByIngredient'}">Search by ingredient</router-link>
        </div>
    </div>
  
</template>

<script>
import { useRouter } from 'vue-router'
import useLogout from '@/tools/useLogout.js'
import getUser from '@/tools/getUser.js'
import { onUpdated, ref, watch } from 'vue'


export default {

setup(){
    const { logout, error, isPending } = useLogout()
    const router = useRouter()
    const { user, getUserDisplayName, userName } = getUser()
    // const userUpdated = ref(null)
    const handleClick = async () =>{
        console.log('start...')
        await logout()
        if(!error.value){
            console.log('USER is out')
            router.push({name: 'Login'})
        }
    }
    // watch(user, () => {
    //     if(user){
    //        location.reload()
    //         router.push({name: 'Home'}) 
            
    //     }
    // })


    // watch(user, () => {
    //     console.log('step 1')
    //     if(user.value && user.value.displayName){
    //         userUpdated.value = user.value
    //         console.log('if 1: ', user.value.displayName)
    //         }else {
    //             userUpdated.value = null
    //         }    
    // })
    // watch( userUpdated, ()=>{
    //     console.log('step 2')
    //     if(user.value){
    //         if(user.value.displayName) {
    //             console.log('if 2: ', user.value.displayName)
    //             document.getElementById("userNameDiv").innerText = "Welcome " + user.value.displayName
    //             }else{
    //                 document.getElementById("userNameDiv").innerText = "Welcome " + user.value.email
    //             }
    //     }else{
    //         document.getElementById("userNameDiv").innerText = 'Welcome Chef'
    //     }
    // })
    
    const handleClickTest = () => {
        console.log('USER name: ',user.value.displayName)
    }
    return { error, isPending, handleClick, user, handleClickTest, getUserDisplayName, userName }
}
}
</script>

<style scoped>
.navbar {
    padding: 16px 10px 0px 10px;
    margin-bottom: 30px;
    background:  rgb(255, 255, 255);
    box-shadow: 2px 2px 8px rgba(50,50,50,0.5);
    box-sizing: border-box;
    
}
.btnNav{
    font-size: 0.8em;
    padding: 6px 20px;
    margin: 0px;
    background: var(--button1);
    color: white;
    display: block;
    width: 100%;
    box-shadow: inset 0px 2px 30px rgba(1, 121, 168, 0.76) ; 
}
.btnNav:hover{
    background: var(--button2);
    color: white;
    box-shadow: none; 
}
.links2{
    text-align: center;
    margin:0 auto 0 auto;
    max-width: 1200px;
    padding: 0;
    
    display: flex;
    justify-content: space-between;
    background:  rgb(255, 255, 255);
}

nav {
    display: flex;
    align-items: center;
    max-width: 1200px;
    margin: 0px auto;
    padding-bottom: 15px;
}
nav .navHeader {
    margin-left: 20px;
}
nav .links {
    margin-left: auto;
}
nav .links {
    margin-left: auto;
    display: flex;
    align-items: center;
}
nav .play{
    transition:all ease 2.2s;
}
nav .play:hover {
      color: rgb(0, 160, 226);
      transition:all ease 0.2s;
}
.links span {
    font-size: 14px;
    display: inline-block;
    margin-left: 16px;
    padding-left: 16px;
    border-left: 1px solid rgb(0, 160, 226);
    color: rgb(0, 160, 226);
}

nav .links a, button {
    margin-left:16px;
    font-size: 14px;
}
nav img {
    max-height: 120px;
}
.blue{
    font-weight: 600;
}
nav h1{
    font-size: 2.0em;
}
.userphoto{
    margin-left: 20px;
    border-radius: 50%;
    border: none;
    max-height: 60px;
}

</style>

