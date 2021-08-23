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

class ActiveVisitListView(APIView):
    permission_classes = [IsAuthenticated]

    def get(self, request):
        """
        Get list of all active visits.
        """
        visits = Visit.objects.filter(end=None)

        serializer = VisitSerializer(visits, many=True)
        return Response(serializer.data, status=status.HTTP_200_OK)