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

from ..students.models import Student

class VisitListView(APIView):
    permission_classes = [IsAuthenticated]

    def get(self, request):
        """
        Get list of all visits.
        """
        visits = Visit.objects.all()

        serializer = VisitSerializer(visits, many=True)
        return Response(serializer.data, status=status.HTTP_200_OK)

class SignInView(APIView):
    def post(self, request):
        """
        Start a new session.
        """
        try:
            student = Student.objects.get(student_id=request.data.get('student_id'))
        except Student.DoesNotExist:
            return Response({"detail": "Student not found."}, status=status.HTTP_404_NOT_FOUND)

        serializer = VisitSerializer(data={'student': student.id, 'start': datetime.now()})

        if serializer.is_valid():
            serializer.save()

            return Response(serializer.data, status=status.HTTP_201_CREATED)
        return Response(serializer.errors, status=status.HTTP_400_BAD_REQUEST)

class SignOutView(APIView):
    def post(self, request):
        """
        End a session.
        """
        try:
            student = Student.objects.get(student_id=request.data.get('student_id'))
        except Student.DoesNotExist:
            return Response({"detail": "Student not found."}, status=status.HTTP_404_NOT_FOUND)

        try:
            visit = Visit.objects.filter(end=None).get(student=student.id)
        except Visit.DoesNotExist:
            return Response({"detail": "No active visit found for this user."}, status=status.HTTP_404_NOT_FOUND)
        
        visit.end_visit()

        return Response(status.HTTP_200_OK)