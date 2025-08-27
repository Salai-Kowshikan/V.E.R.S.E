from fastapi import FastAPI, status, Request
from fastapi.responses import JSONResponse
from routes import index
from middleware.logger import log_requests

app = FastAPI(
    title="V.E.R.S.E API",
    version="1.0.0",
    docs_url="/api/docs",
    redoc_url="/api/redoc",
    openapi_url="/api/openapi.json"
)

# log - middleware
app.middleware("http")(log_requests)

# Routers
app.include_router(index.router, prefix="/api", tags=["General"])

# Root route
@app.get("/")
def read_main_root():
    return JSONResponse(
        status_code=status.HTTP_200_OK,
        content={"message": "Welcome welcome!! VERSE API"}
    )
#hey you
# Catch-all route
@app.get("/{full_path:path}")
def catch_all(full_path: str):
    return JSONResponse(
        status_code=status.HTTP_404_NOT_FOUND,
        content={"message": "Route not found", "path": full_path}
    )
