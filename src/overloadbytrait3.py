def get_color(x: Any):
    try:
        get_color_ = x.get_color
    except AttributeError:
        return 'transparent'
    else:
        return get_color_()
