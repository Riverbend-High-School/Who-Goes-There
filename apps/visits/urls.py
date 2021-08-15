from django.urls import path

from . import views

urlpatterns = [
    path('', views.VisitListView.as_view()),
    path('signin/', views.SignInView.as_view()),
    path('signout/', views.SignOutView.as_view()),
]