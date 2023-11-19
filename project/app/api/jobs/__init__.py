from flask import Blueprint

bp = Blueprint('jobs', __name__, url_prefix="/jobs")

from . import routes