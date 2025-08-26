import time
import logging
from logging.handlers import TimedRotatingFileHandler
from fastapi import Request
import os

# Ensure log directory exists
LOG_DIR = "log"
os.makedirs(LOG_DIR, exist_ok=True)

# Configure logging (daily rotation)
handler = TimedRotatingFileHandler(
    filename=os.path.join(LOG_DIR, "verse.log"),
    when="midnight",
    interval=1,
    backupCount=7,
    encoding="utf-8"
)
formatter = logging.Formatter("%(asctime)s - %(levelname)s - %(message)s")
handler.setFormatter(formatter)

logger = logging.getLogger("verse_logger")
logger.setLevel(logging.INFO)
logger.addHandler(handler)

# Middleware function
async def log_requests(request: Request, call_next):
    start_time = time.time()
    response = await call_next(request)
    duration = time.time() - start_time
    log_message = f"{request.method} {request.url} - {response.status_code} in {duration:.2f}s"
    logger.info(log_message)
    return response
