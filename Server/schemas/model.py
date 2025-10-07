from pydantic import BaseModel
from typing import Optional

class ModelCreate(BaseModel):
    vectorFormat: Optional[str] = None

class ModelResponse(BaseModel):
    id: str
    userId: str
    vectorFormat: Optional[str] = None