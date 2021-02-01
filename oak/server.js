// oak is inspired by Koa.
import {
  Application,
  Context,
  Router
} from 'https://deno.land/x/oak@v6.5.0/mod.ts';
import {v4} from 'https://deno.land/std@0.85.0/uuid/mod.ts';
//import { swaggerDoc } from "https://deno.land/x/deno_swagger_doc/mod.ts";

const PORT = 1234;
const dogs = {};
const id = v4.generate();
dogs[id] = {id, name: 'Comet', breed: 'Whippet'};

/*
const swaggerDefinition = {
  info: {
    title: "Dogs", // required
    version: "1.0.0", // required
    description: "REST services for dogs",
  },
  host: `localhost:8000`, // Host (optional)
  basePath: "/", // Base path (optional)
};

const options = {
  swaggerDefinition,
  apis: ["./server.ts", "./parameters.yaml"],
};

// Initialize swagger-jsdoc -> returns validated swagger spec in json format
const swaggerSpec = swaggerDoc(options);

app.use(async (context, next) => {
  if (context.request.url.pathname === "/swagger.json") {
    context.response.headers.set("Content-Type", "application/json");
    context.response.status = 200;
    context.response.body = swaggerSpec;
  } else {
    await next();
  }
});
*/

function sendJson(context, value) {
  context.response.headers.set('Content-Type', 'application/json');
  context.response.body = JSON.stringify(value);
}

function deleteAllDogs(context) {
  dogs = {};
}

function getAllDogs(context) {
  sendJson(context, Object.values(dogs));
}

function getDog(context) {
  const {id} = context.params;
  const dog = dogs[id];
  if (dog) {
    sendJson(context, dog);
  } else {
    context.response.status = 404;
  }
}

async function createDog(context) {
  const body = await context.request.body();
  const {breed, name} = await body.value;
  const id = v4.generate();
  const dog = {id, breed, name};
  dogs[id] = dog;
  context.response.status = 201;
  sendJson(context, dog);
}

async function updateDog(context) {
  const {id} = context.params;
  if (dogs[id]) {
    const body = await context.request.body();
    const dog = await body.value;
    dogs[id] = dog;
    sendJson(context, dog);
  } else {
    context.response.status = 404;
  }
}

function deleteDog(context) {
  const {id} = context.params;
  const exists = Boolean(dogs[id]);
  if (exists) delete dogs[id];
  context.response.status = exists ? 204 : 404;
}

const router = new Router();
router
  .delete('/dog', deleteAllDogs)
  .delete('/dog/:id', deleteDog)
  .get('/dog', getAllDogs)
  .get('/dog/:id', getDog)
  .post('/dog', createDog)
  .put('/dog/:id', updateDog);
// patch is also supported.

const app = new Application();

// Request logging
/*
app.use(async (ctx, next) => {
  await next();
  console.log(`${ctx.request.method} ${ctx.request.url}`);
});
*/

app.use(router.routes());
app.use(router.allowedMethods());
app.addEventListener('listen', () => {
  console.log('listening on port', PORT);
});
await app.listen({port: PORT});
