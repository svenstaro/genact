
"""All registered module classes."""
all_modules = {}

class MetaModule(type):
    """A metaclass to register modules in the module dictionary, using their
    __modulename__ as key.
    """
    def __new__(cls, name, bases, attrs):
        class_instance = type.__new__(cls, name, bases, attrs)

        if name != "Module":
            if not class_instance.__modulename__:
                raise TypeError("Module class {} needs a __modulename__.".format(name))

            all_modules[class_instance.__modulename__] = class_instance

        return class_instance

class Module(metaclass=MetaModule):
    __modulename__ = None

    def run(self):
        pass
