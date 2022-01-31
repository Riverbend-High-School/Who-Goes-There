from rest_framework import serializers

from wgt_backend.utils import CustomDurationField
from ..visits.models import Visit
from ..students.models import Student


class StatisticsSerializer(serializers.ModelSerializer):
    curr_hours = CustomDurationField()
    prev_hours = CustomDurationField()
    prev_prev_hours = CustomDurationField()

    class Meta:
        model = Student
        fields = ['student_id', 'name', 'curr_hours', 'prev_hours', 'prev_prev_hours']


class TopStudentsSerializer(serializers.ModelSerializer):
    hours = CustomDurationField()

    class Meta:
        model = Student
        fields = ['name', 'hours']

class DailyConnectionsSerializer(serializers.Serializer):
    day = serializers.DateField()
    value = serializers.SerializerMethodField()

    def get_value(self, obj):
        return obj.get('value').total_seconds() / 3600
