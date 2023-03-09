const mongoose = require('mongoose')
const express = require('express')
const app = express()

app.use(express.json())

app.get('/', (req, res) => {
    res.send("h")
})

mongoose.connect('mongodb://10.0.0.213:10001/?readPreference=primary&ssl=false&directConnection=true').then(() => console.log('Connected to database!'));;

app.listen(9898, () => {
  console.log("ToastDB server is listening on port: 9898")
})