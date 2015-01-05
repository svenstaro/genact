
"""All registered module classes."""
all_modules = {}

class MetaModule(type):
    """A metaclass to register modules in the module dictionary, using their
    modulename as key.
    """
    def __new__(cls, name, bases, attrs):
        class_instance = type.__new__(cls, name, bases, attrs)

        if name != "Module":
            if not class_instance.modulename:
                raise TypeError("Module class {} needs a modulename.".format(name))

            all_modules[class_instance.modulename] = class_instance

        return class_instance

class Module(metaclass=MetaModule):
    modulename = None

    def run(self):
        pass
