from beanie import Document
from pydantic import EmailStr, Field
from typing import Optional
from utils.password import hash_password

class User(Document):
    email: EmailStr = Field(..., unique=True)
    hashed_password: str
    
    class Settings:
        name = "users"
        
    @classmethod
    async def create_user(cls, email: str, password: str):
        """Create a new user with hashed password"""
        hashed_password = hash_password(password)
        user = cls(email=email, hashed_password=hashed_password)
        await user.insert()
        return user
    
    @classmethod
    async def get_by_email(cls, email: str):
        """Get user by email"""
        return await cls.find_one(cls.email == email)