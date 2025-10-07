from fastapi.security import HTTPBearer
import jwt
from datetime import datetime, timedelta
from typing import Optional
from fastapi import Depends, HTTPException, status
from fastapi.security import HTTPAuthorizationCredentials
from config.settings import settings
from models.user import User

security = HTTPBearer()

def create_access_token(data: dict, expires_delta: Optional[timedelta] = None) -> str:
    """Create a JWT access token"""
    to_encode = data.copy()
    if expires_delta:
        expire = datetime.utcnow() + expires_delta
    else:
        expire = datetime.utcnow() + timedelta(weeks=settings.jwt_access_token_expire_weeks)
    
    to_encode.update({"exp": expire})
    encoded_jwt = jwt.encode(to_encode, settings.jwt_secret_key, algorithm=settings.jwt_algorithm)
    return encoded_jwt

def verify_token(token: str) -> Optional[str]:
    """Verify and decode a JWT token, returns the email if valid, None otherwise"""
    try:
        payload = jwt.decode(token, settings.jwt_secret_key, algorithms=[settings.jwt_algorithm])
        email: str = payload.get("sub")
        return email
    except jwt.ExpiredSignatureError:
        return None
    except jwt.InvalidTokenError:
        return None

async def get_current_user(credentials: HTTPAuthorizationCredentials = Depends(security)) -> User:
    """
    JWT Authentication dependency function.
    Use this with FastAPI Depends to protect endpoints.
    
    Usage:
    @app.get("/protected")
    async def protected_endpoint(current_user: User = Depends(get_current_user)):
        return {"user": current_user.email}
    """
    credentials_exception = HTTPException(
        status_code=status.HTTP_401_UNAUTHORIZED,
        detail="Could not validate credentials",
        headers={"WWW-Authenticate": "Bearer"},
    )
    
    try:
        token = credentials.credentials
        
        email = verify_token(token)
        if email is None:
            raise credentials_exception

        user = await User.get_by_email(email)
        if user is None:
            raise credentials_exception
            
        return user
        
    except Exception as e:
        raise credentials_exception

def verify_jwt_auth(credentials: HTTPAuthorizationCredentials = Depends(security)) -> bool:
    """
    Simple JWT validation function that returns True if token is valid, False otherwise.
    Use this when you just need to check if a token is valid without getting the user.
    
    Usage:
    @app.get("/check-auth")
    async def check_auth(is_valid: bool = Depends(verify_jwt_auth)):
        return {"authenticated": is_valid}
    """
    try:
        token = credentials.credentials
        email = verify_token(token)
        return email is not None
    except:
        return False