from . import bp

@bp.errorhandler(404)
@bp.route('/', defaults={'path': ''})
@bp.route('/<path:path>')
def spa_index_admin(path):
    return bp.send_static_file("index.html")