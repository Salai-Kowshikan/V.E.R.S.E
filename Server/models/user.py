from beanie import Document
from pydantic import EmailStr


class User(Document):
    email: EmailStr
    
    class Settings:
        name = "users"