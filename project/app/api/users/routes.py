from . import bp
from flask import jsonify, request, session
from ...extensions import db, bcrypt, cache
from ...models.users import User, user_have_permission

@cache.cached(timeout=300)
@bp.route("/have-permission/<permission>", methods=["GET"])
def have_permission(permission):
    user_id = session.get("user_id")
    print(session.get("user_id"))
    print(session.items())
        
    if not user_id:
        return jsonify({"message": "Nenhum usuário conectado nessa sessão"}), 401
    
    if user_have_permission(user_id, permission):
        return {"condition":True}, 200
    
    else:
        return {"condition":False}, 200
    
@bp.route("/register", methods=["POST"])
def register_user():
    username = request.json["username"]
    email = request.json["email"]
    password = request.json["password"]

    user_exists = User.query.filter_by(email=email).first() is not None

    if user_exists:
        return jsonify({"message": "Usuário já existe"}), 409

    hashed_password = bcrypt.generate_password_hash(password)
    new_user = User(username=username, email=email, password=hashed_password)
    db.session.add(new_user)
    db.session.commit()
    
    session["user_id"] = new_user.id

    return jsonify({
        "id": new_user.id,
        "email": new_user.email
    })

@bp.route("/login", methods=['POST'])
def login():
    email = request.json["email"]
    password = request.json["password"]

    user = User.query.filter_by(email=email).first()

    if user is None:
        return jsonify({"message": "Esse usuário não existe"}), 401
    
    if not bcrypt.check_password_hash(user.password, password):
        return jsonify({"message": "Senha incorreta"}), 401
    
    session["user_id"] = user.id
    print(session.get("user_id"))

    return jsonify({
        "id": user.id,
        "username": user.username,
        "email": user.email
    })

@cache.cached(timeout=300)
@bp.route("/me", methods=['GET'])
def me():
    user_id = session.get("user_id")

    if user_id:
        user = User.query.filter_by(id=user_id).first()
        return jsonify({
            "id": user.id,
            "username": user.username,
            "email": user.email
        })
    
    else:
        return jsonify({"message": "Nenhum usuário conectado nessa sessão"}), 401



@bp.route("/logout", methods=['POST'])
def logout():
    session.pop("user_id")
    return "", 200
