from flask import Blueprint

bp = Blueprint('admin', __name__, static_folder="../../../dist/admin/", static_url_path="/admin/", subdomain="admin")

from . import routes