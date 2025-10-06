import boto3
import os
from typing import Optional, Union, Dict, Any
from botocore.exceptions import ClientError, NoCredentialsError
from pathlib import Path
from config.settings import settings


class CloudflareR2Manager:
    """
    Manager class for Cloudflare R2 operations using boto3.
    """
    
    def __init__(self, account_id: str, access_key_id: str, secret_access_key: str, 
                 bucket_name: str, region: str = "auto"):
        """
        Initialize Cloudflare R2 client.
        
        Args:
            account_id: Cloudflare account ID
            access_key_id: R2 access key ID
            secret_access_key: R2 secret access key
            bucket_name: R2 bucket name
            region: AWS region (default: "auto" for Cloudflare R2)
        """
        self.account_id = account_id
        self.bucket_name = bucket_name
        
        # Cloudflare R2 endpoint
        endpoint_url = f"https://{account_id}.r2.cloudflarestorage.com"
        
        try:
            self.s3_client = boto3.client(
                's3',
                endpoint_url=endpoint_url,
                aws_access_key_id=access_key_id,
                aws_secret_access_key=secret_access_key,
                region_name=region
            )
        except Exception as e:
            raise ConnectionError(f"Failed to initialize R2 client: {e}")


def get_r2_manager() -> CloudflareR2Manager:
    """
    Get R2 manager instance using configuration settings.
    
    Returns:
        CloudflareR2Manager: Initialized R2 manager
        
    Raises:
        ValueError: If required configuration values are missing
    """
    # Check if all required R2 settings are configured
    if not all([
        settings.r2_account_id,
        settings.r2_access_key_id,
        settings.r2_secret_access_key,
        settings.r2_bucket_name
    ]):
        missing_settings = []
        if not settings.r2_account_id:
            missing_settings.append('r2_account_id')
        if not settings.r2_access_key_id:
            missing_settings.append('r2_access_key_id')
        if not settings.r2_secret_access_key:
            missing_settings.append('r2_secret_access_key')
        if not settings.r2_bucket_name:
            missing_settings.append('r2_bucket_name')
        
        raise ValueError(f"Missing required R2 configuration: {missing_settings}")
    
    return CloudflareR2Manager(
        account_id=settings.r2_account_id,
        access_key_id=settings.r2_access_key_id,
        secret_access_key=settings.r2_secret_access_key,
        bucket_name=settings.r2_bucket_name
    )


def add_file_to_r2(local_file_path: Union[str, Path], r2_key: str, 
                   metadata: Optional[Dict[str, str]] = None,
                   content_type: Optional[str] = None,
                   r2_manager: Optional[CloudflareR2Manager] = None) -> bool:
    """
    Upload a local file to Cloudflare R2.
    
    Args:
        local_file_path: Path to the local file to upload
        r2_key: The key (path) to store the file in R2
        metadata: Optional metadata to attach to the file
        content_type: Optional content type (will be auto-detected if not provided)
        r2_manager: Optional R2 manager instance (will create one if not provided)
        
    Returns:
        bool: True if file was uploaded successfully, False otherwise
        
    Raises:
        FileNotFoundError: If local file doesn't exist
        ClientError: If R2 operation fails
        NoCredentialsError: If R2 credentials are invalid
    """
    try:
        local_file_path = Path(local_file_path)
        
        # Check if local file exists
        if not local_file_path.exists():
            raise FileNotFoundError(f"Local file does not exist: {local_file_path}")
        
        if not local_file_path.is_file():
            raise ValueError(f"Path is not a file: {local_file_path}")
        
        # Get R2 manager
        if r2_manager is None:
            r2_manager = get_r2_manager()
        
        # Prepare upload arguments
        upload_args = {}
        
        if metadata:
            upload_args['Metadata'] = metadata
            
        if content_type:
            upload_args['ContentType'] = content_type
        else:
            # Auto-detect content type based on file extension
            import mimetypes
            content_type, _ = mimetypes.guess_type(str(local_file_path))
            if content_type:
                upload_args['ContentType'] = content_type
        
        # Upload file to R2
        with open(local_file_path, 'rb') as file_data:
            r2_manager.s3_client.put_object(
                Bucket=r2_manager.bucket_name,
                Key=r2_key,
                Body=file_data,
                **upload_args
            )
        
        print(f"Successfully uploaded {local_file_path} to R2 as {r2_key}")
        return True
        
    except FileNotFoundError as e:
        print(f"Error uploading file to R2: {e}")
        raise
    except (ClientError, NoCredentialsError) as e:
        print(f"R2 error uploading file: {e}")
        raise
    except Exception as e:
        print(f"Unexpected error uploading file to R2: {e}")
        return False


def remove_file_from_r2(r2_key: str, 
                        r2_manager: Optional[CloudflareR2Manager] = None) -> bool:
    """
    Remove a file from Cloudflare R2.
    
    Args:
        r2_key: The key (path) of the file to remove from R2
        r2_manager: Optional R2 manager instance (will create one if not provided)
        
    Returns:
        bool: True if file was removed successfully, False otherwise
        
    Raises:
        ClientError: If R2 operation fails
        NoCredentialsError: If R2 credentials are invalid
    """
    try:
        # Get R2 manager
        if r2_manager is None:
            r2_manager = get_r2_manager()
        
        # Delete file from R2
        r2_manager.s3_client.delete_object(
            Bucket=r2_manager.bucket_name,
            Key=r2_key
        )
        
        print(f"Successfully removed {r2_key} from R2")
        return True
        
    except (ClientError, NoCredentialsError) as e:
        print(f"R2 error removing file: {e}")
        raise
    except Exception as e:
        print(f"Unexpected error removing file from R2: {e}")
        return False


def download_file_from_r2(r2_key: str, local_file_path: Union[str, Path],
                          r2_manager: Optional[CloudflareR2Manager] = None) -> bool:
    """
    Download a file from Cloudflare R2 to local storage.
    
    Args:
        r2_key: The key (path) of the file in R2
        local_file_path: Path where the file should be saved locally
        r2_manager: Optional R2 manager instance (will create one if not provided)
        
    Returns:
        bool: True if file was downloaded successfully, False otherwise
        
    Raises:
        ClientError: If R2 operation fails
        NoCredentialsError: If R2 credentials are invalid
    """
    try:
        local_file_path = Path(local_file_path)
        
        # Get R2 manager
        if r2_manager is None:
            r2_manager = get_r2_manager()
        
        # Create local directory if it doesn't exist
        local_file_path.parent.mkdir(parents=True, exist_ok=True)
        
        # Download file from R2
        r2_manager.s3_client.download_file(
            r2_manager.bucket_name,
            r2_key,
            str(local_file_path)
        )
        
        print(f"Successfully downloaded {r2_key} from R2 to {local_file_path}")
        return True
        
    except (ClientError, NoCredentialsError) as e:
        print(f"R2 error downloading file: {e}")
        raise
    except Exception as e:
        print(f"Unexpected error downloading file from R2: {e}")
        return False


def file_exists_in_r2(r2_key: str, 
                      r2_manager: Optional[CloudflareR2Manager] = None) -> bool:
    """
    Check if a file exists in Cloudflare R2.
    
    Args:
        r2_key: The key (path) of the file in R2
        r2_manager: Optional R2 manager instance (will create one if not provided)
        
    Returns:
        bool: True if file exists in R2, False otherwise
    """
    try:
        # Get R2 manager
        if r2_manager is None:
            r2_manager = get_r2_manager()
        
        # Check if file exists
        r2_manager.s3_client.head_object(
            Bucket=r2_manager.bucket_name,
            Key=r2_key
        )
        return True
        
    except ClientError as e:
        error_code = e.response['Error']['Code']
        if error_code == '404':
            return False
        else:
            print(f"R2 error checking file existence: {e}")
            return False
    except Exception as e:
        print(f"Unexpected error checking file existence in R2: {e}")
        return False


def get_file_info_from_r2(r2_key: str,
                          r2_manager: Optional[CloudflareR2Manager] = None) -> Optional[Dict[str, Any]]:
    """
    Get information about a file in Cloudflare R2.
    
    Args:
        r2_key: The key (path) of the file in R2
        r2_manager: Optional R2 manager instance (will create one if not provided)
        
    Returns:
        dict: File information including size, last modified, content type, etc.
        None: If file doesn't exist or error occurs
    """
    try:
        # Get R2 manager
        if r2_manager is None:
            r2_manager = get_r2_manager()
        
        # Get file metadata
        response = r2_manager.s3_client.head_object(
            Bucket=r2_manager.bucket_name,
            Key=r2_key
        )
        
        return {
            'size': response.get('ContentLength'),
            'last_modified': response.get('LastModified'),
            'content_type': response.get('ContentType'),
            'etag': response.get('ETag'),
            'metadata': response.get('Metadata', {})
        }
        
    except ClientError as e:
        error_code = e.response['Error']['Code']
        if error_code == '404':
            print(f"File not found in R2: {r2_key}")
        else:
            print(f"R2 error getting file info: {e}")
        return None
    except Exception as e:
        print(f"Unexpected error getting file info from R2: {e}")
        return None
