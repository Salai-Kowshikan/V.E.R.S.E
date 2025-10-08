from beanie import Document, Link, Indexed
from typing import Optional
from datetime import datetime
from enum import Enum
from pydantic import Field, validator
from .user import User

class ValidationStatus(str, Enum):
    PENDING = "pending"
    APPROVED = "approved"
    REJECTED = "rejected"
    IN_PROGRESS = "in_progress"

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
    status: ValidationStatus = ValidationStatus.PENDING
    createdAt: datetime = Field(default_factory=datetime.utcnow)
    

