from django.contrib import admin

from .models import *


@admin.register(Student)
class StudentAdmin(admin.ModelAdmin):
    list_display = [field.name for field in Student._meta.get_fields()]
