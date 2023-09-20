from flask import Flask

app = Flask(__name__)

@app.route('/', methods=["GET"]) # methods=["GET"] can be skipped, it is default
def hello_world():
    return f"Hello, World!"

@app.route('/bug')
def hello_bug():
    # let's see how it is rendered in debug mode
    raise Exception("Intentional error")

@app.route('/<name>')
def hello(name):
    name = name or "World"
    return f"Hello, {name}!"

if __name__ == "__main__":
    app.run(port=3000, debug=True)
