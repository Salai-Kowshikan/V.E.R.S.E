from models.user import User
from schemas.user import UserRegistration, UserLogin, UserResponse, Token
from utils.password import verify_password 
from utils.auth import create_access_token
from fastapi import HTTPException, status
from datetime import timedelta
from config.settings import settings

async def get_all_users():
    users = await User.find_all().to_list()
    return [{"id": str(user.id), "email": user.email} for user in users]


async def register_user(user_data: UserRegistration) -> UserResponse:
    """Register a new user"""
    # Check if user already exists
    existing_user = await User.get_by_email(user_data.email)
    if existing_user:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Email already registered"
        )
    
    # Create new user
    user = await User.create_user(user_data.email, user_data.password)
    
    return UserResponse(id=str(user.id), email=user.email)


async def authenticate_user(email: str, password: str) -> User:
    """Authenticate user credentials"""
    user = await User.get_by_email(email)
    if not user:
        return False
    if not verify_password(password, user.hashed_password):
        return False
    return user


async def login_user(user_data: UserLogin) -> Token:
    """Login user and return JWT token"""
    user = await authenticate_user(user_data.email, user_data.password)
    if not user:
        raise HTTPException(
            status_code=status.HTTP_401_UNAUTHORIZED,
            detail="Incorrect email or password",
            headers={"WWW-Authenticate": "Bearer"},
        )
    
    # Create access token
    access_token_expires = timedelta(weeks=settings.jwt_access_token_expire_weeks)
    access_token = create_access_token(
        data={"sub": user.email}, expires_delta=access_token_expires
    )
    
    # Calculate expires_in in seconds (1 week = 7 * 24 * 60 * 60 seconds)
    expires_in = int(access_token_expires.total_seconds())
    
    return Token(
        access_token=access_token,
        token_type="bearer",
        expires_in=expires_in
    )