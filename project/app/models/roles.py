from ..extensions import db
from app.models.permissions import Permission

role_permissions = db.Table(
    'role_permissions',
    db.Column('role_id', db.Integer, db.ForeignKey('role.id')),
    db.Column('permission_id', db.Integer, db.ForeignKey('permission.id'))
)

class Role(db.Model):
    id = db.Column(db.Integer, primary_key=True, unique=True)
    title = db.Column(db.String(150), nullable=False)
    description = db.Column(db.Text, nullable=False)
    permissions = db.relationship('Permission', secondary=role_permissions, backref="role")

    def __repr__(self):
        return f'<Role "{self.title}">'

    def as_dict(self):
        role = {c.name: getattr(self, c.name) for c in self.__table__.columns}
        return role
    
def checkin_role(name, description, permissions):
    role = Role.query.filter_by(title=name).first()

    if not role:
        new_role = Role(title=name, description=description)
        for permission in permissions:
            permission = Permission.query.filter_by(title=permission).first()
            if permission: new_role.permissions.append(permission)

        db.session.add(new_role)

    db.session.commit()