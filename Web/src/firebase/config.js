import firebase from 'firebase/app'
import 'firebase/firestore'
import 'firebase/auth'
import 'firebase/storage'

const firebaseConfig = {
  apiKey: "AIzaSyAEvKe333EQOReDc3o7-jiBv54hqPNDKnU",
  authDomain: "cookbook-307109.firebaseapp.com",
  projectId: "cookbook-307109",
  storageBucket: "cookbook-307109.appspot.com",
  messagingSenderId: "458563897259",
  appId: "1:458563897259:web:7b5490ed8d23d72e5ff4ee",
  measurementId: "G-809KTHP1C7"
};

  // init firebase
  firebase.initializeApp(firebaseConfig)

  // init services
  const db = firebase.firestore()
  const dbAuth = firebase.auth()
  const dbStorage = firebase.storage()

  // timestamp

  const timestamp = firebase.firestore.FieldValue.serverTimestamp

  export { db, dbAuth, dbStorage, timestamp}
