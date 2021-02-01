from flask import Flask, abort, jsonify, request
from flask_cors import CORS

import time

# Serve static files found in the public directory.
app = Flask(__name__, static_folder='public', static_url_path='')
CORS(app)  # for cross-origin resource sharing

dogs = {
    1: {
        'id': 'some-id', 'breed': 'Whippet', 'name': 'Comet'
    }
}

@app.route('/dog')
def get_dogs():
    return jsonify(dogs.values())

@app.route('/dog/<id>')
def get_dog(id):
    print("id =", id)
    print("dogs =", dogs)
    return jsonify(dogs[id])

@app.route('/dog', methods=['POST'])
def create_dog():
    dog = request.get_json()  # from body
    id = round(time.time() * 1000)
    dog['id'] = id
    dogs[id] = dog
    return jsonify(dog)

@app.route('/dog/<id>', methods=['PUT'])
def update_dog(id):
    id = int(id)
    if id in dogs:
        dog = request.get_json()  # from body
        dog['id'] = id
        dogs[id] = dog
        return jsonify(dog)
    else:
        abort(404)

@app.route('/dog/<id>', methods=['DELETE'])
def delete_dog(id):
    id = int(id)
    if id in dogs:
        del dogs[id]
        return ''
    else:
        abort(404)
