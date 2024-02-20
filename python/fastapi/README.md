# Fast API

## 基本

mainコード  

```python
# import
from fastapi import FastAPI

# entry
app = FastAPI()

@app.get("/")
async def root():
    return {"message": "Hello World"}
```

起動  
(appはmainコードの変数)  

```sh
uvicorn main:app --reload
```
