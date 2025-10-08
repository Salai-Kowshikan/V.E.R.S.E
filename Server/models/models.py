from beanie import Document, Link
from typing import Optional
from datetime import datetime
from enum import Enum
from pydantic import Field
from .user import User

class ValidationStatus(str, Enum):
    PENDING = "pending"
    COMPLETED = "completed"

class Model(Document):
    userId: Link[User]
    vectorFormat: str
    name: str 
    description: Optional[str] = None
    createdAt: datetime = Field(default_factory=datetime.utcnow)
    updatedAt: datetime = Field(default_factory=datetime.utcnow)
    
    class Settings:
        name = "models"

class ValidationRequest(Document):
    modelId: Link[Model]
    verifierId: Link[User]
    elfFileUrl: str 
    jsonUrl : Optional[str] = None
    status: ValidationStatus = ValidationStatus.PENDING
    proofHash: str
    createdAt: datetime = Field(default_factory=datetime.utcnow)
    

