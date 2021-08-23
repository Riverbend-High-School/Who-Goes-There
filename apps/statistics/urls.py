from django.urls import path

from . import views

urlpatterns = [  
    path('', views.StatisticsView.as_view()),
    path('daily_stats/<int:year>/', views.DailyStatisticsView.as_view()),
    path('top/', views.TopStudentsView.as_view()),
]