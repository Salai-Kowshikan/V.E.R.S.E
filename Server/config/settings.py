from pydantic_settings import BaseSettings

class Settings(BaseSettings):
    # Database Configuration
    database_url: str = "mongodb://localhost:27017"
    database_name: str = "testdb"
    
    # API Configuration
    api_title: str = "V.E.R.S.E API"
    api_version: str = "1.0.0"

    debug: bool = False

    class Config:
        env_file = ".env"
        env_file_encoding = "utf-8"

settings = Settings()