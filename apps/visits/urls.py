from django.urls import path

from . import views

urlpatterns = [
    path('', views.VisitListView.as_view()),
    path('active/', views.ActiveVisitListView.as_view()),
]