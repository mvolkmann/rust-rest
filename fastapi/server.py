from fastapi import FastAPI, Response, status
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from typing import Optional
import time
import uuid

# JSON in request bodies of POST and PUT requests
# is validated against this type definition.
# When validation fails, the response status
# is set to 422 Unprocessable Entity.
class Dog(BaseModel):
    id: Optional[str] = None
    breed: str
    name: str

id = str(uuid.uuid4())
dogs = {}
dogs[id] = {'id': id, 'breed': 'Whippet', 'name': 'Comet'}

app = FastAPI()
app.add_middleware(
    CORSMiddleware,
    allow_origins=['*'],
    allow_credentials=True,
    allow_methods=['*'],
    allow_headers=['*'])

@app.get('/dog')
def get_dogs():
    return list(dogs.values())

@app.get('/dog/{id}')
def get_dog(id: str):
    if id in dogs:
        return dogs[id]
    else:
        return Response(status_code=status.HTTP_404_NOT_FOUND)

@app.post('/dog', status_code=201)
def create_dog(dog: Dog):
    id = str(uuid.uuid4())
    # dog['id'] = id # Why can't the dog object be modified?
    dict = dog.dict()
    dict['id'] = id
    dogs[id] = dict
    return dict

@app.put('/dog/{id}')
def update_dog(dog: Dog, id: str):
    if id in dogs:
        # dog['id'] = id # Why can't the dog object be modified?
        dict = dog.dict()
        dict['id'] = id
        dogs[id] = dict
        return dict
    else:
        return Response(status_code=status.HTTP_404_NOT_FOUND)

@app.delete('/dog/{id}')
def delete_dog(id: str):
    if id in dogs:
        del dogs[id]
        return Response(status_code=status.HTTP_204_NO_CONTENT)
    else:
        return Response(status_code=status.HTTP_404_NOT_FOUND)
