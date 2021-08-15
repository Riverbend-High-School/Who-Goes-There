from django.contrib import admin

from .models import *


@admin.register(Visit)
class VisitAdmin(admin.ModelAdmin):
    list_display = [field.name for field in Visit._meta.get_fields()]
