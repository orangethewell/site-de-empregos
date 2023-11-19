from ..extensions import db
import json

class Job(db.Model):
    id = db.Column(db.Integer, primary_key=True)
    # Short Description
    title = db.Column(db.String(150), nullable=False)
    company = db.Column(db.String(150), nullable=False)
    branch = db.Column(db.String(150), nullable=False)
    opportunities = db.Column(db.Integer)

    # Long Description
    activities = db.Column(db.Text, nullable=False)
    requirements = db.Column(db.Text, nullable=False)

    def __repr__(self):
        return f'<Job "{self.title}">'

    def as_dict(self):
        job = {c.name: getattr(self, c.name) for c in self.__table__.columns}
        return job

    def as_formatted_dict(self):
        job = self.as_dict()
        
        if job["activities"] != None:
            job["activities"] = json.loads(job["activities"])
    
        if job["requirements"] != None:
            job["requirements"] = json.loads(job["requirements"])

        return job