from fastapi import APIRouter, HTTPException, status, Depends
from controller.user import get_all_users, register_user, login_user
from schemas.user import UserRegistration, UserLogin, UserResponse, Token
from utils.auth import get_current_user
from models.user import User
router = APIRouter()


@router.post("/register", response_model=UserResponse, status_code=status.HTTP_201_CREATED)
async def register(user_data: UserRegistration):
    """Register a new user"""
    return await register_user(user_data)


@router.post("/login", response_model=Token)
async def login(user_data: UserLogin):
    """Login user and return JWT token with 1 week expiry"""
    return await login_user(user_data)


@router.get("",response_model=list[UserResponse])
async def list_users( current_user:User = Depends(get_current_user)):
    """Get all users"""
    print(current_user+" accessed list_users endpoint")
    return await get_all_users()
