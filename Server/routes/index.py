from fastapi import APIRouter, HTTPException, status
from config.settings import settings
from models.user import User
router = APIRouter()

@router.get("")
def read_root():
    return {"message": "Yay !! you have hit the root endpoint .Check the /api/health endpoint for health status. /api/docs for API documentation."}


@router.get("/health")
async def health_check():
    try:
 
        await User.find().limit(1).to_list()
        
        return {
            "status": "healthy",
            "database": settings.database_name,
            "message": "Server is up and running. Database connection is working"
        }
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail=f"Database connection failed: {str(e)}"
        )
