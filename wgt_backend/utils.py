import os
import requests
from rest_framework.fields import DurationField
from django.core.files.storage import FileSystemStorage
from django.conf import settings

class CustomDurationField(DurationField):
    def to_representation(self, value):
        minutes, seconds = divmod(value.total_seconds(), 60)
        hours, minutes = divmod(minutes, 60)

        string = f'{int(hours):02}:{int(minutes):02}:{int(seconds):02}'

        return string