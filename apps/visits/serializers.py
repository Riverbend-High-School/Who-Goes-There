from rest_framework import serializers

from wgt_backend.utils import CustomDurationField
from .models import Visit
from ..students.models import Student

class VisitSerializer(serializers.ModelSerializer):
    class Meta:
        model = Visit
        fields = '__all__'

