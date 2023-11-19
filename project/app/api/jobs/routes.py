from . import bp
from flask import jsonify, request, session
from app.extensions import db
from app.models.jobs import Job
from app.models.users import user_have_permission

import json

@bp.route("/", methods=['GET', 'POST'])
def jobs():
    if request.method == 'POST':
        user_id = session.get("user_id")
        print(user_id)
        
        if not user_id:
            return jsonify({"error": "Não autorizado"}), 401
        
        if not user_have_permission(user_id, "EditJobs"):
            return jsonify({"error": "Não autorizado"}), 401

        data = request.get_json()
        print(data)
        new_job = Job(
            title=data["title"],
            company=data["company"],
            branch=data["branch"],
            opportunities=data["opportunities"],

            activities=json.dumps(data["activities"]),
            requirements=json.dumps(data["requirements"])
        )
        db.session.add(new_job)
        db.session.commit()
        return new_job.as_formatted_dict()

    if request.method == 'GET':
        jobs = Job.query.all()

        response = []
        for job in jobs:
            response.append(job.as_formatted_dict())

        return response
    
@bp.route("/count", methods=['GET'])
def jobs_count():
    count = Job.query.count()

    return jsonify(count)
    
@bp.route("/<id>", methods=['GET', 'PATCH', 'DELETE'])
def jobs_by_id(id):
    if request.method == 'PATCH':
        user_id = session.get("user_id")
        
        if not user_id:
            return jsonify({"error": "Não autorizado"}), 401
        
        if not user_have_permission(user_id, "EditJobs"):
            return jsonify({"error": "Não autorizado"}), 401
        
        data = request.get_json()
        job_details = Job(
            title=data["title"],
            company=data["company"],
            branch=data["branch"],
            opportunities=data["opportunities"],

            activities=json.dumps(data["activities"]),
            requirements=json.dumps(data["requirements"])
        )
        unident_job_details = job_details.as_dict()
        del unident_job_details["id"]
        Job.query.filter_by(id=id).update(unident_job_details)
        db.session.commit()
        return job_details.as_dict()
    

    if request.method == 'DELETE':
        user_id = session.get("user_id")
        
        if not user_id:
            return jsonify({"error": "Não autorizado"}), 401
        
        if not user_have_permission(user_id, "EditJobs"):
            return jsonify({"error": "Não autorizado"}), 401
        
        row = Job.query.filter_by(id=id).delete()
        db.session.commit()
        return str(row)

    if request.method == 'GET':
        job = Job.query.filter_by(id = id).first()
        return job.as_formatted_dict()
