import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'
import Login from '../views/auth/Login.vue'
import Signup from '../views/auth/Signup.vue'
import MealDetails from '../components/MealDetails.vue'
import SearchByName from '../components/SearchByName.vue'
import SearchByArea from '../components/SearchByArea.vue'
import SearchByCategory from '../components/SearchByCategory.vue'
import SearchByIngredient from '../components/SearchByIngredient.vue'
import Categories from '../views/Categories.vue'
import About from '../views/About.vue'
import UserList from '../views/UserList.vue'

// route guard
import { dbAuth } from '../firebase/config.js'

const requiredAuth = (to, from, next) => {
  let user = dbAuth.currentUser
  if (!user){
    next({name: 'Login'})
  } else {
    next()
  }
}

const requiredNoUser = (to, from, next) => {
  let user = dbAuth.currentUser
  if (!user){
    next()
  } 
   else {
     next(from)
     alert("You are currently logged in. Please log out first")
   }
}

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home,
    beforeEnter: requiredAuth
  },
    {
    path: '/about',
    name: 'About',
    component: About,
    beforeEnter: requiredAuth
  },
  {
    path: '/login',
    name: 'Login',
    component: Login,
    beforeEnter: requiredNoUser
  },
  {
    path: '/signup',
    name: 'Signup',
    component: Signup,
    beforeEnter: requiredNoUser
  },
  {
    path: '/allCategories',
    name: 'Categories',
    component: Categories,
    beforeEnter: requiredAuth
  },
  {
    path: '/favorites',
    name: 'UserList',
    component: UserList,
    beforeEnter: requiredAuth

  },
  {
    path: '/meals/:id',
    name: 'MealDetails',
    component: MealDetails,
    beforeEnter: requiredAuth,
    props: true
  },
  {
    path: '/byName',
    name: 'SearchByName',
    component: SearchByName,
    beforeEnter: requiredAuth,
  },
  {
    path: '/byAreas',
    name: 'SearchByArea',
    component: SearchByArea,
    beforeEnter: requiredAuth,
  },
  {
    path: '/byCategories',
    name: 'SearchByCategory',
    component: SearchByCategory,
    beforeEnter: requiredAuth,
  },
  {
    path: '/ingredients',
    name: 'SearchByIngredient',
    component: SearchByIngredient,
    beforeEnter: requiredAuth,
  },
  
]

const router = createRouter({
  history: createWebHistory(process.env.BASE_URL),
  routes
})

export default router
