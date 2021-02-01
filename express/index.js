'use strict';
const express = require('express');
const {v4: uuid} = require('uuid');

const app = express();
app.use(express.json());

const dogs = {};
const id = uuid();
dogs[id] = {id, name: 'Comet', breed: 'Whippet'};

function sendJson(res, value) {
  res.set('Content-Type', 'application/json');
  res.send(JSON.stringify(value));
}

app.get('/dog', (req, res) => {
  sendJson(res, Object.values(dogs));
});

app.get('/dog/:id', (req, res) => {
  const {id} = req.params;
  const dog = dogs[id];
  if (dog) {
    sendJson(res, dog);
  } else {
    res.sendStatus(404);
  }
});

app.post('/dog', (req, res) => {
  const dog = req.body;
  const id = uuid();
  dog.id = id;
  dogs[id] = dog;
  res.status(201);
  sendJson(res, dog);
});

app.put('/dog/:id', (req, res) => {
  const {id} = req.params;
  const dog = req.body;
  if (dogs[id]) {
    dogs[id] = dog;
    sendJson(res, dog);
  } else {
    res.sendStatus(404);
  }
});

app.delete('/dog/:id', (req, res) => {
  const {id} = req.params;
  const exists = Boolean(dogs[id]);
  if (exists) delete dogs[id];
  res.sendStatus(exists ? 204 : 404);
});

const port = 1234;
app.listen(port, () => console.log('listening on port', port));
