#!/bin/python3
from klein import resource, route

@route('/')
def hello(request):
    return "Hello, world!"
