from flask import Blueprint
from .jobs import bp as jobs_bp
from .users import bp as users_bp

bp = Blueprint('api', __name__, url_prefix="/api")

bp.register_blueprint(jobs_bp)
bp.register_blueprint(users_bp)