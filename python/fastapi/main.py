from enum import Enum
from typing import Union, List
from fastapi import FastAPI, Form
from pydantic import BaseModel

# uvicorn main:app --reload
app = FastAPI()

fake_items_db = [{"item_name": "Foo"}, {"item_name": "Bar"}, {"item_name": "Baz"}]


# Enum
class ModelName(str, Enum):
    alexnet = "alexnet"
    resnet = "resnet"
    lenet = "lenet"


# get data simply.
@app.get("/")
def read_root():
    return {"Hello": "World"}


# get by path parameter.
# item_id must be `int`.
@app.get("/items/{item_id}")
async def read_item(item_id: int):
    return {"item_id": item_id}


# Enum can be used as path parameter
@app.get("/models/{model_name}")
async def get_model(model_name: ModelName):
    if model_name is ModelName.alexnet:
        return {"model_name": model_name, "message": "Deep Learning FTW!"}

    if model_name.value == "lenet":
        return {"model_name": model_name, "message": "LeCNN all the images"}

    return {"model_name": model_name, "message": "Have some residuals"}


# if you use path like `home/johndoe/myfile.txt`,
# specify `path` keyword.
@app.get("/files/{file_path:path}")
async def read_file(file_path: str):
    return {"file_path": file_path}


# item_id must be `str`.
# needy required and `str`.
# skip and limit can be skip by adding default value.
@app.get("/items/{item_id}")
async def read_user_item(
    item_id: str, needy: str, skip: int = 0, limit: Union[int, None] = None
):
    item = {"item_id": item_id, "needy": needy, "skip": skip, "limit": limit}
    return item


# path parameter and query parameter
@app.get("/items/{item_id}")
async def read_item(item_id: str, q: Union[str, None] = None, short: bool = False):
    item = {"item_id": item_id}
    if q:
        item.update({"q": q})
    if not short:
        item.update(
            {"description": "This is an amazing item that has a long description"}
        )
    return item


class Item(BaseModel):
    name: str
    description: Union[str, None] = None
    price: float
    tax: Union[float, None] = None
    tags: List[str] = []


# query parameter
@app.get("/items/")
async def read_item(skip: int = 0, limit: int = 10):
    return fake_items_db[skip : skip + limit]


# if you specify the class inheritance BaseModel as argument,
#   read request body as json
#   convert to appropriate type
#   validate data
#   convert to Item
@app.put("/items/{item_id}")
def update_item(item_id: int, item: Item):
    return {"item_name": item.name, "item_id": item_id}


# json request
@app.post("/items/")
async def create_item(item: Item):
    return item


# form request
@app.post("/items2/")
async def create_item2(
    name: str = Form(),
    price: str = Form(),
    description: Union[str, None] = Form(),
    tax: Union[float, None] = Form(),
):
    item = Item(name=name, description=description, price=price, tax=tax)
    return item
