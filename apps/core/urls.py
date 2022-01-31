from django.urls import path

from . import views

urlpatterns = [
    path('hello/', views.HelloView.as_view(), name='hello'),
    path('hello_token/', views.HelloTokenView.as_view(), name='hello_token'),
]