from flask_sqlalchemy import SQLAlchemy
db = SQLAlchemy()

from flask_bcrypt import Bcrypt
bcrypt = Bcrypt()

from flask_session import Session
server_session = Session()

from flask_migrate import Migrate
migrate = Migrate()

from flask_caching import Cache
cache = Cache()