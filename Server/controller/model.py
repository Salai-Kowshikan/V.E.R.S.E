from models.models import Model
from models.user import User
from schemas.model import ModelCreate, ModelResponse
from fastapi import HTTPException, status
from typing import Dict, Any

async def create_model(model_data: ModelCreate, current_user: User) -> ModelResponse:
    """Create a new model for the authenticated user"""
    try:
        # Create new model linked to the current user
        model = Model(
            userId=current_user,
            vectorFormat=model_data.vectorFormat
        )
        
        # Save the model to the database
        await model.insert()
        
        return ModelResponse(
            id=str(model.id),
            userId=str(current_user.id),
            vectorFormat=model.vectorFormat
        )
        
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to create model: {str(e)}"
        )

async def get_user_models(current_user: User):
    """Get all models for the authenticated user"""
    try:
        models = await Model.find(Model.userId.id == current_user.id).to_list()
        return [
            ModelResponse(
                id=str(model.id),
                userId=str(current_user.id),
                vectorFormat=model.vectorFormat
            )
            for model in models
        ]
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to retrieve models: {str(e)}"
        )