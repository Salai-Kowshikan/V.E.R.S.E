from fastapi import APIRouter, HTTPException, status, Depends
from controller.model import create_model, get_user_models
from schemas.model import ModelCreate, ModelResponse
from utils.auth import get_current_user
from models.user import User
from typing import List

router = APIRouter()

@router.post("", response_model=ModelResponse, status_code=status.HTTP_201_CREATED)
async def create_new_model(
    model_data: ModelCreate,
    current_user: User = Depends(get_current_user)
):
    """Create a new model for the authenticated user"""
    return await create_model(model_data, current_user)

@router.get("", response_model=List[ModelResponse])
async def get_models(
    current_user: User = Depends(get_current_user)
):
    """Get all models for the authenticated user"""
    return await get_user_models(current_user)