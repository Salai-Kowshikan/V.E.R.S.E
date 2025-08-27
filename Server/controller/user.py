from models.user import User


async def create_user(email: str):
    user = User(email=email)
    await user.insert()
    return {"id": str(user.id), "email": user.email}


async def get_all_users():
    users = await User.find_all().to_list()
    return [{"id": str(user.id), "email": user.email} for user in users]