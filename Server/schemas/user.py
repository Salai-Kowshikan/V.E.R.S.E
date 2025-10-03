from pydantic import BaseModel, EmailStr
from typing import Optional

class UserRegistration(BaseModel):
    email: EmailStr
    password: str

class UserLogin(BaseModel):
    email: EmailStr
    password: str

class UserResponse(BaseModel):
    id: str
    email: EmailStr

class Token(BaseModel):
    access_token: str
    token_type: str
    expires_in: int  # seconds

class TokenData(BaseModel):
    email: Optional[str] = None