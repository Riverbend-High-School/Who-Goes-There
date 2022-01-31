from rest_framework import serializers

from wgt_backend.utils import CustomDurationField
from .models import Visit
from ..students.models import Student

class VisitSerializer(serializers.ModelSerializer):
    class Meta:
        model = Visit
        fields = '__all__'

class ActiveVisitSerializer(serializers.ModelSerializer):
    student_info = serializers.ReadOnlyField()
    start_simple = serializers.ReadOnlyField()
    unix_time = serializers.ReadOnlyField()

    class Meta:
        model = Visit
        fields = ['student_info', 'start', 'start_simple', 'unix_time']

