from ..extensions import db

class Permission(db.Model):
    id = db.Column(db.Integer, primary_key=True, unique=True)
    title = db.Column(db.String(150), nullable=False)
    description = db.Column(db.Text, nullable=False)

    def __repr__(self):
        return f'<Permission "{self.title}">'

    def as_dict(self):
        permission = {c.name: getattr(self, c.name) for c in self.__table__.columns}
        return permission

def checkin_permission(name, description):
    permission = Permission.query.filter_by(title=name).first()

    if not permission:
        new_permission = Permission(title=name, description=description)
        db.session.add(new_permission)

    db.session.commit()