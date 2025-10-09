from pydantic import BaseModel, computed_field
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
    name :str 
    description: Optional[str] = None
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
    model :Optional[ModelResponse] = None
    jsonUrl : Optional[str] = None
    proofHash :str
    status: ValidationStatus
    createdAt: datetime

    @classmethod
    def from_validation_request(cls, validation_request, model=None):
        """Create ValidationRequestResponse with public URLs appended"""
        from utils.file import construct_public_url
        
        return cls(
            id=str(validation_request.id),
            modelId=str(validation_request.modelId.id if hasattr(validation_request.modelId, 'id') else validation_request.modelId),
            verifierId=str(validation_request.verifierId.id if hasattr(validation_request.verifierId, 'id') else validation_request.verifierId),
            elfFileUrl=construct_public_url(validation_request.elfFileUrl) if validation_request.elfFileUrl else "",
            jsonUrl=construct_public_url(validation_request.jsonUrl) if validation_request.jsonUrl else None,
            proofHash=validation_request.proofHash,
            status=validation_request.status,
            createdAt=validation_request.createdAt,
            model=model
        )




class ValidationRequestUpdate(BaseModel):
    status: Optional[ValidationStatus] = None
    comments: Optional[str] = None

class ModelWithValidationsResponse(BaseModel):
    id: str
    userId: str
    name: str
    description: Optional[str] = None
    vectorFormat: Optional[str] = None
    createdAt: datetime
    updatedAt: datetime
    validationRequests: List[ValidationRequestResponse] = []

class UserModelsWithValidationsResponse(BaseModel):
    models: List[ModelWithValidationsResponse]