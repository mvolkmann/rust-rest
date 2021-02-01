from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional
import time

# JSON in request bodies of POST and PUT requests
# is validated against this type definition.
# When validation fails, the response status
# is set to 422 Unprocessable Entity.
class Dog(BaseModel):
    id: Optional[int] = None
    breed: str
    name: str

dogs = {
    1: {
        'id': 1, 'breed': 'Whippet', 'name': 'Comet'
    }
}

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=['*'],
    allow_credentials=True,
    allow_methods=['*'],
    allow_headers=['*'])

@app.get('/dog')
def all_dogs():
    return dogs.values()

@app.post('/dog', response_model=str)
def create_dog(dog: Dog):
    id = round(time.time() * 1000)
    # dog['id'] = id # Why can't the dog object be modified?
    dict = dog.dict()
    dict['id'] = id
    dogs[id] = dict
    return str(id)

@app.put('/dog/{id}', response_model=str)
def update_dog(dog: Dog, id: int):
    if id in dogs:
        # dog['id'] = id # Why can't the dog object be modified?
        dict = dog.dict()
        dict['id'] = id
        dogs[id] = dict
        return ''
    else:
        abort(404)

@app.delete('/dog/{id}')
def delete_dog(id: int):
    id = int(id)
    if id in dogs:
        del dogs[id]
        return ''
    else:
        abort(404)
