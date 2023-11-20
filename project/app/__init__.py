from flask import Blueprint, Flask
from flask_cors import CORS
import redis
import os

basedir = os.path.abspath(os.path.dirname(__file__))

class Config:
    SECRET_KEY = os.environ.get('SECRET_KEY')
    
    SQLALCHEMY_DATABASE_URI = os.environ.get('DATABASE_URI')\
        or 'sqlite:///' + os.path.join(basedir, 'app.db')
    SQLALCHEMY_TRACK_MODIFICATIONS = False
    SQLALCHEMY_ECHO = False

    SESSION_TYPE="redis"
    SESSION_PERMANENT = False
    SESSION_USE_SIGNER = True
    SESSION_REDIS = redis.from_url("redis://127.0.0.1:6379")
    SESSION_COOKIE_SAMESITE = 'None'

    CACHE_TYPE = "RedisCache"
    CACHE_DEFAULT_TIMEOUT = 300
    CACHE_REDIS_URL = "redis://127.0.0.1:6379"

class ProductionConfig(Config):
    SERVER_NAME = "vagasemaraxa.com.br"
    SESSION_COOKIE_SECURE = True

class DevelopmentConfig(Config):
    SERVER_NAME = "vagasemaraxa.com:80"
    DEBUG=True

    SESSION_COOKIE_HTTPONLY = True
    SESSION_COOKIE_SECURE = False

from .extensions import db, bcrypt, server_session, migrate, cache
from .models.permissions import checkin_permission
from .models.roles import checkin_role
from .models.users import checkin_admin_user

def create_app(config_class=ProductionConfig()):
    app = Flask(__name__, subdomain_matching=True)
    CORS(app, 
        expose_headers=["Content-Type", "Access-Control-Allow-Credentials"],
        supports_credentials=True
    )
    if app.debug:
        app.config.from_object(DevelopmentConfig())
    else:
        app.config.from_object(config_class)
    print(app.config["SERVER_NAME"])
    app.url_map.default_subdomain = "www"

    # Initialize Flask extensions here
    db.init_app(app)
    
    migrate.init_app(app)

    bcrypt.init_app(app)

    server_session.init_app(app)

    cache.init_app(app)

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

    bp = Blueprint('main', __name__, url_prefix="/", subdomain="www", static_url_path="/", static_folder="../../dist")

    @bp.errorhandler(404)
    @bp.route('/', defaults={'path': ''})
    @bp.route('/<path:path>')
    def spa_index(path):
        return bp.send_static_file("index.html")

    app.register_blueprint(bp)

    return app

if __name__ == "__main__":
    create_app(DevelopmentConfig()).run(debug=True)