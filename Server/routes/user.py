from fastapi import APIRouter
from controller.user import create_user, get_all_users

router = APIRouter()


@router.post("")
async def add_user(email: str):
    return await create_user(email)


@router.get("")
async def list_users():
    return await get_all_users()
