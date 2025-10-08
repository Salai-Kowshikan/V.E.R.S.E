from models.models import Model, ValidationRequest, ValidationStatus
from models.user import User
from schemas.model import (
    ModelCreate, ModelResponse, 
    ValidationRequestResponse, ModelWithValidationsResponse,
    UserModelsWithValidationsResponse
)
from fastapi import HTTPException, status, UploadFile
from typing import List
from beanie import PydanticObjectId
from datetime import datetime
import uuid
import tempfile
import os
from utils.file import get_r2_manager, add_file_to_r2

async def create_model(model_data: ModelCreate, current_user: User) -> ModelResponse:
    """Create a new model for the authenticated user"""
    try:
        # Create new model linked to the current user
        model = Model(
            userId=current_user,
            vectorFormat=model_data.vectorFormat,
            name=model_data.name,
            description=model_data.description
        )
        
        # Save the model to the database
        await model.insert()
        
        return ModelResponse(
            id=str(model.id),
            userId=str(current_user.id),
            vectorFormat=model.vectorFormat,
            createdAt=model.createdAt,
            updatedAt=model.updatedAt
        )
        
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to create model: {str(e)}"
        )

async def get_user_models(current_user: User):
    """Get all models by user"""
    try:
        models = await Model.find(Model.userId.id == current_user.id).to_list()
        return [
            ModelResponse(
                id=str(model.id),
                userId=str(current_user.id),
                vectorFormat=model.vectorFormat,
                createdAt=model.createdAt,
                updatedAt=model.updatedAt
            )
            for model in models
        ]
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to retrieve models: {str(e)}"
        )


async def get_all_models_controller():
    """Get all models """
    try:
        models = await Model.find().to_list()
        return [
            ModelResponse(
                id=str(model.id),
                userId=str(model.userId.ref.id),
                vectorFormat=model.vectorFormat,
                createdAt=model.createdAt,
                updatedAt=model.updatedAt
            )
            for model in models
        ]

    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to retrieve models: {str(e)}"
        )


async def create_validation_request_with_file(
    model_id: str,
    elf_file: UploadFile,
    current_user: User
) -> ValidationRequestResponse:
    """Create a new validation request with ELF file upload"""
    try:
        # Verify the model exists
        model = await Model.get(model_id)
        if not model:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail="Model not found"
            )
        print(str(model.userId.ref.id))
        print(str(current_user.id))
        # Prevent users from creating validation requests for their own models
        if str(model.userId.ref.id) == str(current_user.id):
            raise HTTPException(
                status_code=status.HTTP_403_FORBIDDEN,
                detail="You cannot create validation requests for your own models"
            )
        print("different user")
        # Validate file type (optional - you can add specific validations)
        # if not elf_file.filename.endswith('.elf'):
        #     raise HTTPException(
        #         status_code=status.HTTP_400_BAD_REQUEST,
        #         detail="File must be an ELF file"
        #     )
        
        # Generate unique filename for R2
        file_extension = os.path.splitext(elf_file.filename)[1]
        unique_filename = f"elf_files/{uuid.uuid4()}{file_extension}"
        
        # Create temporary file to save uploaded content
        with tempfile.NamedTemporaryFile(delete=False, suffix=file_extension) as temp_file:
            try:
                # Read and write file content
                content = await elf_file.read()
                temp_file.write(content)
                temp_file.flush()
                
                # Upload to Cloudflare R2
                r2_manager = get_r2_manager()
                success = add_file_to_r2(
                    local_file_path=temp_file.name,
                    r2_key=unique_filename,
                    metadata={
                        'original_filename': elf_file.filename,
                        'uploaded_by': str(current_user.id),
                        'model_id': model_id,
                        'upload_date': datetime.utcnow().isoformat()
                    },
                    content_type='application/octet-stream',
                    r2_manager=r2_manager
                )
                
                if not success:
                    raise HTTPException(
                        status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                        detail="Failed to upload file to cloud storage"
                    )
                
                # Construct file URL (adjust based on your R2 configuration)
                file_url = f"{unique_filename}"
                
            finally:
                try:
                    os.unlink(temp_file.name)
                except OSError:
                    pass  # File already deleted or doesn't exist
        
        # Create validation request with current user as verifier
        validation_request = ValidationRequest(
            modelId=model,
            verifierId=current_user,
            elfFileUrl=file_url
        )
        
        await validation_request.insert()
        
        return ValidationRequestResponse(
            id=str(validation_request.id),
            modelId=str(validation_request.modelId.id),
            verifierId=str(validation_request.verifierId.id),
            elfFileUrl=validation_request.elfFileUrl,
            status=validation_request.status,
            createdAt=validation_request.createdAt,
        )
        
    except HTTPException:

        raise
    except Exception as e:
        print(e)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to create validation request: {str(e)}"
        )

async def get_model_validation_requests(model_id: str, current_user: User) -> List[ValidationRequestResponse]:
    """Get all validation requests for a specific model"""
    try:
        print(f"Getting validation requests for model_id: {model_id}")
        print(f"Current user ID: {current_user.id}")
        
        # Verify the model exists and belongs to the current user
        model = await Model.get(model_id)
        if not model:
            print(f"Model not found for ID: {model_id}")
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail="Model not found"
            )
        
        if str(model.userId.ref.id) != str(current_user.id):
            print(f"User mismatch - Model owner: {model.userId.ref.id}, Current user: {current_user.id}")
            raise HTTPException(
                status_code=status.HTTP_403_FORBIDDEN,
                detail="You can only view validation requests for your own models"
            )
        
        # Get validation requests for this model
        validation_requests = await ValidationRequest.find(
            ValidationRequest.modelId.id == PydanticObjectId(model_id)
        ).to_list()

       
        
        return [
            ValidationRequestResponse(
                id=str(vr.id),
                modelId=str(vr.modelId.ref.id),
                verifierId=str(vr.verifierId.ref.id),
                elfFileUrl=vr.elfFileUrl,
                status=vr.status,
                createdAt=vr.createdAt,
            )
            for vr in validation_requests
        ]
        
    except HTTPException:
        print("HTTPException caught, re-raising")
        raise
    except Exception as e:
        print(f"Unexpected error in get_model_validation_requests: {str(e)}")
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to retrieve validation requests: {str(e)}"
        )

async def get_user_models_with_validations(current_user: User) -> UserModelsWithValidationsResponse:
    """Get all user models with their validation requests"""
    try:
        # Get user's models
        models = await Model.find(Model.userId.id == current_user.id).to_list()
        
        models_with_validations = []
        
        for model in models:
            # Get validation requests for each model
            validation_requests = await ValidationRequest.find(
                ValidationRequest.modelId.id == model.id
            ).to_list()
            
            validation_responses = [
                ValidationRequestResponse(
                    id=str(vr.id),
                    modelId=str(vr.modelId.ref.id),
                    verifierId=str(vr.verifierId.ref.id),
                    elfFileUrl=vr.elfFileUrl,
                    status=vr.status,
                    createdAt=vr.createdAt
                )
                for vr in validation_requests
            ]
            
            models_with_validations.append(
                ModelWithValidationsResponse(
                    id=str(model.id),
                    userId=str(model.userId.ref.id),
                    vectorFormat=model.vectorFormat,
                    createdAt=model.createdAt,
                    updatedAt=model.updatedAt,
                    validationRequests=validation_responses
                )
            )
        
        return UserModelsWithValidationsResponse(models=models_with_validations)
        
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to retrieve models with validations: {str(e)}"
        )


