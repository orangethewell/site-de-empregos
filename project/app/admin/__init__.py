from flask import Blueprint

bp = Blueprint('admin', __name__, url_prefix="/admin", static_folder="../../dist/admin", static_url_path="/")

from . import routes
