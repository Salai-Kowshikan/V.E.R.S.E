from pydantic import BaseModel
from typing import Optional, List
from datetime import datetime
from enum import Enum

# Define ValidationStatus here to avoid circular import
class ValidationStatus(str, Enum):
    PENDING = "pending"
    COMPLETED = "completed"

class ModelCreate(BaseModel):
    vectorFormat: str
    name: str 
    description: Optional[str] = None

class ModelResponse(BaseModel):
    id: str
    userId: str
    vectorFormat: Optional[str] = None
    createdAt: datetime
    updatedAt: datetime

class ValidationRequestCreate(BaseModel):
    modelId: str
    comments: Optional[str] = None

class ValidationRequestCreateWithFile(BaseModel):
    modelId: str
    comments: Optional[str] = None

class ValidationRequestResponse(BaseModel):
    id: str
    modelId: str
    verifierId: str
    elfFileUrl: str
    status: ValidationStatus
    createdAt: datetime

class ValidationRequestUpdate(BaseModel):
    status: Optional[ValidationStatus] = None
    comments: Optional[str] = None

class ModelWithValidationsResponse(BaseModel):
    id: str
    userId: str
    vectorFormat: Optional[str] = None
    createdAt: datetime
    updatedAt: datetime
    validationRequests: List[ValidationRequestResponse] = []

class UserModelsWithValidationsResponse(BaseModel):
    models: List[ModelWithValidationsResponse]