from fastapi import APIRouter, HTTPException, status, Depends, UploadFile, File, Form
from controller.model import (
    create_model, get_user_models, create_validation_request,
    get_model_validation_requests, get_user_models_with_validations,
    update_validation_request, get_user_verification_requests,
    create_validation_request_with_file, get_models_available_for_validation
)
from schemas.model import (
    ModelCreate, ModelResponse, ValidationRequestCreate,
    ValidationRequestResponse, UserModelsWithValidationsResponse,
    ValidationRequestUpdate
)
from utils.auth import get_current_user
from models.user import User
from typing import List, Optional

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

@router.post("/validation-request", response_model=ValidationRequestResponse, status_code=status.HTTP_201_CREATED)
async def create_new_validation_request(
    model_id: str = Form(...),
    elf_file: UploadFile = File(...),
    comments: Optional[str] = Form(None),
    current_user: User = Depends(get_current_user)
):
    """Create a new validation request for a model with ELF file upload"""
    return await create_validation_request_with_file(model_id, elf_file, current_user, comments)

@router.get("/{model_id}/validation-requests", response_model=List[ValidationRequestResponse])
async def get_validation_requests(
    model_id: str,
    current_user: User = Depends(get_current_user)
):
    """Get all validation requests for a specific model"""
    return await get_model_validation_requests(model_id, current_user)

@router.get("/validations", response_model=UserModelsWithValidationsResponse)
async def get_models_with_validations(
    current_user: User = Depends(get_current_user)
):
    """Get all user models with their validation requests"""
    return await get_user_models_with_validations(current_user)

@router.put("/validation-request/{validation_id}", response_model=ValidationRequestResponse)
async def update_validation_request_status(
    validation_id: str,
    update_data: ValidationRequestUpdate,
    current_user: User = Depends(get_current_user)
):
    """Update a validation request (only verifiers can update)"""
    return await update_validation_request(validation_id, update_data, current_user)

@router.get("/verification-requests", response_model=List[ValidationRequestResponse])
async def get_verification_requests(
    current_user: User = Depends(get_current_user)
):
    """Get all validation requests assigned to the current user for verification"""
    return await get_user_verification_requests(current_user)

@router.get("/available-for-validation", response_model=List[ModelResponse])
async def get_models_available_for_validation(
    current_user: User = Depends(get_current_user)
):
    """Get all models that the current user can create validation requests for (models owned by other users)"""
    return await get_models_available_for_validation(current_user)