from ..extensions import db, bcrypt
from uuid import uuid4
from app.models.roles import Role

user_roles = db.Table(
    'user_roles',
    db.Column('user_id', db.String(32), db.ForeignKey('user.id')),
    db.Column('role_id', db.Integer, db.ForeignKey('role.id'))
)

def get_uuid():
    return uuid4().hex

class User(db.Model):
    id = db.Column(db.String(32), primary_key=True, unique=True, default=get_uuid)
    username = db.Column(db.String(150), nullable=False)
    email = db.Column(db.String(150), nullable=False)
    password = db.Column(db.String(150), nullable=False)
    roles = db.relationship("Role", secondary=user_roles, backref="user")

    def __repr__(self):
        return f'<User "{self.title}">'

    def as_dict(self):
        user = {c.name: getattr(self, c.name) for c in self.__table__.columns if c.name != "roles"}
        return user

def checkin_admin_user():
    admin_role = Role.query.filter_by(title='Admin').first()

    admin_users = User.query.filter(User.roles.any(title='Admin')).count()

    if admin_users == 0:
        admin_user = User(
            username='Administrador',
            email='admin@localhost',
            password=bcrypt.generate_password_hash('admin'),
        )

        admin_user.roles.append(admin_role)

        db.session.add(admin_user)
        db.session.commit()

def user_have_permission(user_id, permission_title):
    user = User.query.filter_by(id=user_id).first()

    for role in user.roles:
        for permission in role.permissions:
            if permission.title == permission_title:
                return True
    
    return False