import os
from datetime import datetime
from django.utils import timezone
from django.http import Http404
from rest_framework import status
from rest_framework.exceptions import PermissionDenied
from rest_framework.generics import get_object_or_404
from rest_framework.permissions import IsAuthenticated
from rest_framework.response import Response
from rest_framework.views import APIView


from .serializers import *
from .statistics import *

from ..students.models import Student

class TopStudentsView(APIView):
    # permission_classes = [IsAuthenticated]

    def get(self, request):
        """
        Get list of controllers sorted by most
        hours for the current month.
        """
        students = get_top_students()
        serializer = TopStudentsSerializer(students, many=True)
        return Response(serializer.data)


class StatisticsView(APIView):
    # permission_classes = [IsAuthenticated]

    def get(self, request):
        """
        Get list of all students along with hours for
        current, previous, and penultimate months.
        """
        hours = get_student_hours()
        serializer = StatisticsSerializer(hours, many=True)
        return Response(serializer.data)


class DailyStatisticsView(APIView):
    # permission_classes = [IsAuthenticated]

    def get(self, request, year):
        """
        Get list of students hours for
        every day of the given year.
        """
        connections = get_daily_statistics(year)
        serializer = DailyConnectionsSerializer(connections, many=True)
        return Response(serializer.data)