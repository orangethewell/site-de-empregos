from flask import Blueprint, Flask
import os

basedir = os.path.abspath(os.path.dirname(__file__))

class Config:
    SECRET_KEY = os.environ.get('SECRET_KEY', 'dev-only-change-me')
    
    SQLALCHEMY_DATABASE_URI = os.environ.get('DATABASE_URI')\
        or 'sqlite:///' + os.path.join(basedir, 'app.db')
    SQLALCHEMY_TRACK_MODIFICATIONS = False
    SQLALCHEMY_ECHO = False

    SESSION_COOKIE_HTTPONLY = True
    SESSION_COOKIE_SAMESITE = 'Lax'

class ProductionConfig(Config):
    SESSION_COOKIE_SECURE = True

class DevelopmentConfig(Config):
    DEBUG = True
    SESSION_COOKIE_SECURE = False

from .extensions import db, bcrypt
from .models.permissions import checkin_permission
from .models.roles import checkin_role
from .models.users import checkin_admin_user

def create_app(config_class=None):
    app = Flask(__name__)
    if config_class is None:
        config_class = (ProductionConfig
                        if os.environ.get('APP_ENV') == 'production'
                        else DevelopmentConfig)
    app.config.from_object(config_class)

    # Initialize Flask extensions here
    db.init_app(app)
    
    bcrypt.init_app(app)

    # Register blueprints here
    from .admin import bp as admin_bp
    app.register_blueprint(admin_bp)

    from .api import bp as api_bp
    app.register_blueprint(api_bp)

    # Handle Permissions

    with app.app_context():
        db.create_all()
        checkin_permission("EditJobs", 
            "The user can add, edit or delete jobs from Jobs Table.")
        checkin_permission("DashboardViewer", 
            "The user can access the Administrator panel and see dashboard and \
            other site areas.")

        checkin_role("Admin",
            "Control the website properties and manage all the things",
            ["EditJobs", "DashboardViewer"])

        checkin_admin_user()

    # Main Blueprint

    bp = Blueprint('main', __name__, url_prefix="/", static_url_path="/", static_folder="../dist")

    @bp.errorhandler(404)
    @bp.route('/', defaults={'path': ''})
    @bp.route('/<path:path>')
    def spa_index(path):
        return bp.send_static_file("index.html")

    app.register_blueprint(bp)

    return app

if __name__ == "__main__":
    create_app(DevelopmentConfig()).run(debug=True)
