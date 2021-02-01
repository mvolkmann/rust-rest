from flask import Flask, abort, jsonify, make_response, request
from flask_cors import CORS

import time
import uuid

# Disable request logging.
import logging
log = logging.getLogger('werkzeug')
log.setLevel(logging.ERROR)

# Serve static files found in the public directory.
app = Flask(__name__, static_folder='public', static_url_path='')
CORS(app)  # for cross-origin resource sharing

id = str(uuid.uuid4())
dogs = {}
dogs[id] = {'id': id, 'breed': 'Whippet', 'name': 'Comet'}

@app.route('/dog')
def get_dogs():
    return jsonify(list(dogs.values()))

@app.route('/dog/<id>')
def get_dog(id):
    if id in dogs:
        return jsonify(dogs[id])
    else:
        abort(404)

@app.route('/dog', methods=['POST'])
def create_dog():
    dog = request.get_json()  # from body
    id = str(uuid.uuid4())
    dog['id'] = id
    dogs[id] = dog
    return make_response(jsonify(dog), 201)

@app.route('/dog/<id>', methods=['PUT'])
def update_dog(id):
    if id in dogs:
        dog = request.get_json()  # from body
        dog['id'] = id
        dogs[id] = dog
        return jsonify(dog)
    else:
        abort(404)

@app.route('/dog/<id>', methods=['DELETE'])
def delete_dog(id):
    if id in dogs:
        del dogs[id]
        return make_response('', 204)  # NO CONTENT
        return ''
    else:
        abort(404)
