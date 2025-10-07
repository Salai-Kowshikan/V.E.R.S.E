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
    vectorFormat: Optional[str] = None
    createdAt: datetime = Field(default_factory=datetime.utcnow)
    updatedAt: datetime = Field(default_factory=datetime.utcnow)
    
    class Settings:
        name = "models"

class ValidationRequest(Document):
    modelId: Link[Model] = Indexed()
    verifierId: Link[User] = Indexed()  # The user who will verify this model
    elfFileUrl: str  # Cloudflare reference to the ELF file
    status: ValidationStatus = ValidationStatus.PENDING
    createdAt: datetime = Field(default_factory=datetime.utcnow)
    verifiedAt: Optional[datetime] = None
    comments: Optional[str] = None
    
    @validator('verifiedAt', always=True)
    def set_verified_at(cls, v, values):
        """Automatically set verifiedAt when status changes to approved/rejected"""
        status = values.get('status')
        if status in [ValidationStatus.APPROVED, ValidationStatus.REJECTED] and v is None:
            return datetime.utcnow()
        elif status == ValidationStatus.PENDING:
            return None
        return v
    
    class Settings:
        name = "validation_requests"