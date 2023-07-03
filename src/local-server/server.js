const { default: axios } = require("axios")
const express = require("express")

const app = express()

app.use(express.json())
app.use(express.urlencoded({ extended: true }))

let temps = 0

app.get("/", (req, res) => {
    console.log(req.body)

    temps++

    if(temps === 2){
        setTimeout(() => {
            return res.send({ data: [
                {
                    projectRef: { key: "10" },
                    number: "",
                    title: ""
                }
            ]})
        }, 1000);
    }

    setTimeout(() => {
        res.status(400).send()
    }, 1000);

})


app.patch("/", (req, res) => {
    console.log(req.body)

    res.send({ number: 5588})
})

app.get("/concorrencia", (req, res) => {
    console.log(req.body)

    res.send("deu certo a requisição")
})

app.listen(3000)