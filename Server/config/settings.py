from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    # Database Configuration
    database_url: str = "mongodb://localhost:27017"
    database_name: str = "testdb"
    
    # API Configuration
    api_title: str = "V.E.R.S.E API"
    api_version: str = "1.0.0"

    # JWT Configuration
    jwt_secret_key: str = "your-secret-key-here"
    jwt_algorithm: str = "HS256"
    jwt_access_token_expire_weeks: int = 1

    # R2 (Cloudflare R2) Configuration
    r2_account_id: str = ""
    r2_access_key_id: str = ""
    r2_secret_access_key: str = ""
    r2_bucket_name: str = ""

    debug: bool = False

    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"
        extra = "ignore"  # This allows extra fields to be ignored rather than causing validation errors

settings = Settings()