from beanie import Document, Link
from typing import Optional
from .user import User

class Model(Document):
    userId : Link[User]
    vectorFormat : Optional[str] = None