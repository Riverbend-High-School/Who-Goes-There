from django.db import models

class Student(models.Model):
    # Required Identification Fields
    name = models.CharField(max_length=255)
    student_id = models.IntegerField()
    
    # Admin Only Comment
    staff_comment_author = models.CharField(max_length=255)
    staff_comment = models.TextField(blank=True)

    # Admin Only Options
    is_banned = models.BooleanField(default=False)
    is_staff = models.BooleanField(default=False)

    def __str__(self):
        return self.name
