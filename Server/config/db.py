from beanie import init_beanie
from motor import motor_asyncio
from .settings import settings


async def init_db():
    try:
        # Import models here to avoid circular imports
        from models.user import User
        from models.models import Model, ValidationRequest
    
        client = motor_asyncio.AsyncIOMotorClient(settings.database_url)
        database = client[settings.database_name]
    
        await init_beanie(database=database, document_models=[User, Model, ValidationRequest])
        print(f"✓ Database connection successful to {settings.database_name}.")

    except Exception as e:
        print(f"✗ Database connection failed: {e}")


async def close_db():
    
    print("✓ Database connection closed.")
