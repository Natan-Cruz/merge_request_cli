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

//  axios({
//     method: "POST",
// url: 'https://multiplier.jetbrains.space/api/http/projects/id:2ZsKnR42KI6t/code-reviews/merge-requests?$fields=number',
//     headers: {
//         'Authorization': "Bearer eyJhbGciOiJSUzUxMiJ9.eyJzdWIiOiJjY3JCMTB1VUp0eiIsImF1ZCI6ImNpcmNsZXQtd2ViLXVpIiwib3JnRG9tYWluIjoibXVsdGlwbGllciIsIm5hbWUiOiJuYXRhLmNydXoiLCJpc3MiOiJodHRwczpcL1wvbXVsdGlwbGllci5qZXRicmFpbnMuc3BhY2UiLCJwZXJtX3Rva2VuIjoiM2g3ZTFqNDZZWFpTIiwicHJpbmNpcGFsX3R5cGUiOiJVU0VSIiwiaWF0IjoxNjg1MTMxOTgzfQ.dIOLL5YDoUwxWjdPcESfyQgj7kNj24xK6sQsz2eXDO2muQmPVhstyrWnTf6VPEKbs3EvkyWEM0yutR8Z2wFcvVVmHFLnuDAm9L5cV5lc0qawYIh64AXQgjvuh02hkD3psxwQ73ZP8l2YI9ftMPsc8B7wpan_NzsewtHIEc99Khw",
//         'Accept': 'application/json',
//         'Content-Type': 'application/json'
//     },
//     data: {
//         "repository": "front",
//         "sourceBranch": "feature/teste",
//         "targetBranch": "milestone/customer",
//         "title": "teste",
//         "reviewers": [
//             {
//             "profileId": "ccrB10uUJtz",
//             "isCodeOwner": true
//             }
//         ]
//     }
//  })
//  .then( response => console.log(response))
//  .catch(console.log)

app.listen(3000)