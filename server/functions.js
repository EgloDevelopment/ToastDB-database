const express = require('express')
const router = express.Router()
const axios = require('axios')
var Datastore = require('nedb')

var databases = new Datastore({filename: './database/databases.json', autoload: true})
var tables = new Datastore({filename: './database/tables.json', autoload: true})
var errors = new Datastore({filename: './database/errors.json', autoload: true})

function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}

// INDEX
router.get('/', (req, res) => {
    res.json({"status":0})
})

// INDEX
router.post('/', (req, res) => {
    res.json({"status":0})
})

// DB FIND
router.post('/find/:table', (req, res) => {
    let table = req.params.table
    tables.find({ name: table }, function(err, docs) {
        if (docs[0] === undefined) {
            res.json({"status":1,"error":"table does not exist"});
            return
        }
    });
    databases.find({ enabled: true }, async function(err, docs) {
        try {
            const JsonAggreagate = []
            for (const val of docs) {
                var url = val.url + "/find/" + table
                let result = await axios.post(url, req.body)
                for (const val of result.data) {
                    JsonAggreagate.push(JSON.parse(val))
                };
            }
            res.json({"status":0,"result":JsonAggreagate});
        } catch (error) {
            res.json({"status":1,"error":error});
            errors.insert({"error":error}, function (err, result) {});
            console.log(error)
        }
    })
})

// DB INSERT
router.post('/insert/:table', (req, res) => {
    let table = req.params.table
    tables.find({ name: table }, function(err, docs) {
        if (docs[0] === undefined) {
            res.json({"status":1,"error":"table does not exist"});
            return
        }
    });
    databases.find({ enabled: true }, async function(err, docs) {
        try {
            let items = docs.length()
            let node = getRandomInt(items + 1)
            var url = val.url[node] + "/insert/" + table
            let result = await axios.post(url, req.body)
            if (result.status === 0) {
                res.json({"status":0});
            } else {
                res.json({"status":1,"error":result.error});
            }
        } catch (error) {
            console.log(error)
            errors.insert({"error":error}, function (err, result) {});
        }
    })
})

// DB DELETE
router.post('/delete/:table', (req, res) => {
    let table = req.params.table
    tables.find({ name: table }, function(err, docs) {
        if (docs[0] === undefined) {
            res.json({"status":1,"error":"table does not exist"});
            return
        }
    });
    databases.find({ enabled: true }, async function(err, docs) {
        try {
            for (const val of docs) {
                var url = val.url + "/delete/" + table
                await axios.post(url, req.body)
            }
            res.json({"status":0});
        } catch (error) {
            res.json({"status":1,"error":error});
            errors.insert({"error":error}, function (err, result) {});
            console.log(error)
        }
    })
})

// CLUSTER NEW
router.post('/new-cluster', (req, res) => {
    tables.find({ name: req.body.name }, function(err, docs) {
        if (docs[0] !== 'undefined') {
            res.json({"status":1,"error":"cluster with same name exists"});
            return;
        }
    });
    databases.insert({ name: req.body.name, enabled: req.body.enabled, url: req.body.url }, async function(err, docs) {
        try {
            var url = req.body.url + "/test"
            let result = await axios.post(url, req.body)
            if (result.data.status === 0) {
                res.json({"status":0});
            } else {
                res.json({"status":1,"error":"created cluster, but could not connect"});
            }
        } catch (error) {
            console.log(error)
            errors.insert({"error":error}, function (err, result) {});
        }
    })
})

module.exports = router
